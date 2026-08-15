use janus_core::{EventSignificance, StateMutation};
use janus_db::PgPool;
use tracing::{debug, info};
use uuid::Uuid;

use crate::error::McpError;
use crate::tools::{
    GetLocationContextArgs, InspectNpcDetailsArgs, LogEventArgs, MoveToLocationArgs, ToolCall,
    ToolExecutionResult, UpdateNpcRelationArgs,
};

pub struct McpExecutor;

impl McpExecutor {
    pub async fn execute_tool(
        pool: &PgPool,
        campaign_id: Uuid,
        turn_number: i32,
        tool_call: &ToolCall,
    ) -> Result<ToolExecutionResult, McpError> {
        debug!(
            tool_name = %tool_call.name,
            campaign_id = %campaign_id,
            turn = turn_number,
            "Executing MCP tool"
        );

        match tool_call.name.as_str() {
            "get_location_context" => {
                let args: GetLocationContextArgs = serde_json::from_value(tool_call.arguments.clone())
                    .map_err(|e| McpError::InvalidArguments {
                        tool: "get_location_context".to_string(),
                        message: e.to_string(),
                    })?;
                Self::execute_get_location_context(pool, campaign_id, args).await
            }
            "inspect_npc_details" => {
                let args: InspectNpcDetailsArgs = serde_json::from_value(tool_call.arguments.clone())
                    .map_err(|e| McpError::InvalidArguments {
                        tool: "inspect_npc_details".to_string(),
                        message: e.to_string(),
                    })?;
                Self::execute_inspect_npc_details(pool, args).await
            }
            "update_npc_relation" => {
                let args: UpdateNpcRelationArgs = serde_json::from_value(tool_call.arguments.clone())
                    .map_err(|e| McpError::InvalidArguments {
                        tool: "update_npc_relation".to_string(),
                        message: e.to_string(),
                    })?;
                Self::execute_update_npc_relation(pool, turn_number, args).await
            }
            "move_to_location" => {
                let args: MoveToLocationArgs = serde_json::from_value(tool_call.arguments.clone())
                    .map_err(|e| McpError::InvalidArguments {
                        tool: "move_to_location".to_string(),
                        message: e.to_string(),
                    })?;
                Self::execute_move_to_location(pool, campaign_id, args).await
            }
            "log_event" => {
                let args: LogEventArgs = serde_json::from_value(tool_call.arguments.clone())
                    .map_err(|e| McpError::InvalidArguments {
                        tool: "log_event".to_string(),
                        message: e.to_string(),
                    })?;
                Self::execute_log_event(pool, campaign_id, args).await
            }
            unknown => Err(McpError::ToolNotFound(unknown.to_string())),
        }
    }

    pub async fn execute_tools(
        pool: &PgPool,
        campaign_id: Uuid,
        turn_number: i32,
        tool_calls: &[ToolCall],
    ) -> Result<Vec<ToolExecutionResult>, McpError> {
        let mut results = Vec::with_capacity(tool_calls.len());
        for call in tool_calls {
            let res = Self::execute_tool(pool, campaign_id, turn_number, call).await?;
            results.push(res);
        }
        Ok(results)
    }

    async fn execute_get_location_context(
        pool: &PgPool,
        campaign_id: Uuid,
        args: GetLocationContextArgs,
    ) -> Result<ToolExecutionResult, McpError> {
        let loc_id = match args.location_id {
            Some(id) => id,
            None => {
                let campaign = janus_db::campaigns::get_by_id(pool, campaign_id)
                    .await?
                    .ok_or_else(|| {
                        McpError::Execution(format!("Campaign {} not found", campaign_id))
                    })?;
                campaign.current_location_id.ok_or_else(|| {
                    McpError::Execution("Campaign has no current location set".to_string())
                })?
            }
        };

        let location = janus_db::locations::get_by_id(pool, loc_id)
            .await?
            .ok_or_else(|| McpError::Execution(format!("Location {} not found", loc_id)))?;

        let edges = janus_db::location_edges::list_connected_edges(pool, loc_id).await?;
        let mut connected_locations = Vec::new();
        for edge in edges {
            let neighbor_id = if edge.source_location_id == loc_id {
                edge.target_location_id
            } else {
                edge.source_location_id
            };
            let neighbor_name = match janus_db::locations::get_by_id(pool, neighbor_id).await? {
                Some(loc) => loc.name,
                None => "Lieu inconnu".to_string(),
            };

            connected_locations.push(serde_json::json!({
                "location_id": neighbor_id,
                "name": neighbor_name,
                "is_locked": edge.is_locked,
                "lock_reason": edge.lock_reason,
                "travel_description": edge.travel_description,
            }));
        }

        let npcs = janus_db::npcs::list_by_location(pool, loc_id).await?;
        let mut npcs_present = Vec::new();
        for npc in npcs {
            let rel = janus_db::npcs::get_relationship(pool, npc.id).await?;
            npcs_present.push(serde_json::json!({
                "npc_id": npc.id,
                "slug": npc.slug,
                "name": npc.name,
                "title": npc.title,
                "relationship": {
                    "affinity": rel.as_ref().map(|r| r.affinity).unwrap_or(0),
                    "trust": rel.as_ref().map(|r| r.trust).unwrap_or(0),
                    "mood": rel.as_ref().map(|r| r.mood.clone()).unwrap_or_else(|| "neutre".to_string()),
                }
            }));
        }

        let result = serde_json::json!({
            "location_id": location.id,
            "slug": location.slug,
            "name": location.name,
            "description": location.description,
            "atmosphere": location.atmosphere,
            "secrets": if args.include_secrets.unwrap_or(false) { location.secrets } else { None },
            "connected_locations": connected_locations,
            "npcs_present": npcs_present,
        });

        info!(location = %location.name, "Executed get_location_context");
        Ok(ToolExecutionResult::success(
            "get_location_context",
            result,
            None,
        ))
    }

    async fn execute_inspect_npc_details(
        pool: &PgPool,
        args: InspectNpcDetailsArgs,
    ) -> Result<ToolExecutionResult, McpError> {
        let npc = janus_db::npcs::get_by_id(pool, args.npc_id)
            .await?
            .ok_or_else(|| McpError::Execution(format!("NPC {} not found", args.npc_id)))?;

        let rel = janus_db::npcs::get_relationship(pool, args.npc_id).await?;

        let result = serde_json::json!({
            "npc_id": npc.id,
            "campaign_id": npc.campaign_id,
            "slug": npc.slug,
            "name": npc.name,
            "title": npc.title,
            "personality_traits": npc.personality_traits,
            "secret_agenda": npc.secret_agenda,
            "background": npc.background,
            "is_alive": npc.is_alive,
            "is_active": npc.is_active,
            "relationship": {
                "affinity": rel.as_ref().map(|r| r.affinity).unwrap_or(0),
                "trust": rel.as_ref().map(|r| r.trust).unwrap_or(0),
                "mood": rel.as_ref().map(|r| r.mood.clone()).unwrap_or_else(|| "neutre".to_string()),
                "last_interaction_turn": rel.as_ref().map(|r| r.last_interaction_turn).unwrap_or(0),
                "interaction_summary": rel.as_ref().and_then(|r| r.interaction_summary.clone()),
            },
            "query_focus": args.query_focus,
        });

        info!(npc = %npc.name, "Executed inspect_npc_details");
        Ok(ToolExecutionResult::success(
            "inspect_npc_details",
            result,
            None,
        ))
    }

    async fn execute_update_npc_relation(
        pool: &PgPool,
        turn_number: i32,
        args: UpdateNpcRelationArgs,
    ) -> Result<ToolExecutionResult, McpError> {
        let npc = janus_db::npcs::get_by_id(pool, args.npc_id)
            .await?
            .ok_or_else(|| McpError::Execution(format!("NPC {} not found", args.npc_id)))?;

        let delta_aff = args.delta_affinity.unwrap_or(0).clamp(-50, 50);
        let delta_tr = args.delta_trust.unwrap_or(0).clamp(-50, 50);

        let updated_rel = janus_db::npcs::update_relationship_deltas(
            pool,
            args.npc_id,
            delta_aff,
            delta_tr,
            args.mood.as_deref(),
            Some(&args.reason),
            turn_number,
        )
        .await?;

        let mutation = StateMutation::RelationshipUpdate {
            npc_id: npc.id,
            npc_name: npc.name.clone(),
            affinity: updated_rel.affinity,
            trust: updated_rel.trust,
            mood: updated_rel.mood.clone(),
            delta_affinity: args.delta_affinity,
            delta_trust: args.delta_trust,
            reason: args.reason.clone(),
        };

        let result = serde_json::json!({
            "npc_id": npc.id,
            "npc_name": npc.name,
            "affinity": updated_rel.affinity,
            "trust": updated_rel.trust,
            "mood": updated_rel.mood,
            "reason": args.reason,
        });

        info!(
            npc = %npc.name,
            affinity = updated_rel.affinity,
            trust = updated_rel.trust,
            "Executed update_npc_relation"
        );
        Ok(ToolExecutionResult::success(
            "update_npc_relation",
            result,
            Some(mutation),
        ))
    }

    async fn execute_move_to_location(
        pool: &PgPool,
        campaign_id: Uuid,
        args: MoveToLocationArgs,
    ) -> Result<ToolExecutionResult, McpError> {
        let target_loc = janus_db::locations::get_by_id(pool, args.target_location_id)
            .await?
            .ok_or_else(|| {
                McpError::Execution(format!(
                    "Target location {} not found",
                    args.target_location_id
                ))
            })?;

        let campaign = janus_db::campaigns::get_by_id(pool, campaign_id)
            .await?
            .ok_or_else(|| {
                McpError::Execution(format!("Campaign {} not found", campaign_id))
            })?;

        let current_loc_id = campaign.current_location_id;

        if current_loc_id != Some(args.target_location_id) {
            if args.force != Some(true) {
                if let Some(src_id) = current_loc_id {
                    let edge = janus_db::location_edges::find_edge_between(
                        pool,
                        src_id,
                        args.target_location_id,
                    )
                    .await?;

                    match edge {
                        None => {
                            return Err(McpError::Execution(format!(
                                "Lieu inaccessible: aucun passage direct entre le lieu actuel et '{}'",
                                target_loc.name
                            )));
                        }
                        Some(e) if e.is_locked => {
                            return Err(McpError::Execution(format!(
                                "Passage verrouillé vers '{}': {}",
                                target_loc.name,
                                e.lock_reason.as_deref().unwrap_or("accès interdit")
                            )));
                        }
                        _ => {}
                    }
                }
            }

            janus_db::campaigns::update_current_location(
                pool,
                campaign_id,
                Some(args.target_location_id),
            )
            .await?;
        }

        let mutation = StateMutation::LocationChange {
            location_id: target_loc.id,
            location_name: target_loc.name.clone(),
            narration_hint: args.movement_narration_hint.clone(),
        };

        let result = serde_json::json!({
            "previous_location_id": current_loc_id,
            "target_location_id": target_loc.id,
            "target_location_name": target_loc.name,
            "movement_narration_hint": args.movement_narration_hint,
        });

        info!(
            target_location = %target_loc.name,
            "Executed move_to_location"
        );
        Ok(ToolExecutionResult::success(
            "move_to_location",
            result,
            Some(mutation),
        ))
    }

    async fn execute_log_event(
        pool: &PgPool,
        campaign_id: Uuid,
        args: LogEventArgs,
    ) -> Result<ToolExecutionResult, McpError> {
        let significance = args
            .significance
            .as_deref()
            .and_then(|s| s.parse::<EventSignificance>().ok())
            .unwrap_or(EventSignificance::Notable);

        let location_id = match args.location_id {
            Some(loc_id) => Some(loc_id),
            None => janus_db::campaigns::get_by_id(pool, campaign_id)
                .await?
                .and_then(|c| c.current_location_id),
        };

        let new_event = janus_db::events::NewNarrativeEvent {
            id: Some(Uuid::new_v4()),
            campaign_id,
            turn_id: None,
            location_id,
            summary: args.summary.clone(),
            significance: Some(significance),
            involved_npc_ids: args.involved_npc_ids,
            tags: args.tags,
        };

        let event = janus_db::events::create(pool, &new_event).await?;

        let mutation = StateMutation::EventLogged {
            event_id: event.id,
            summary: event.summary.clone(),
            significance: event.significance.to_string(),
        };

        let result = serde_json::json!({
            "event_id": event.id,
            "summary": event.summary,
            "significance": event.significance.to_string(),
        });

        info!(
            event_id = %event.id,
            summary = %event.summary,
            "Executed log_event"
        );
        Ok(ToolExecutionResult::success(
            "log_event",
            result,
            Some(mutation),
        ))
    }
}
