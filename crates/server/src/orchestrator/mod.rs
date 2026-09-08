pub mod context;
pub mod prompts;
pub mod sse;

pub use sse::to_sse_event;

use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use janus_core::{StateMutation, TurnStreamEvent};
use janus_llm::types::{DirectorBriefing, ToolCall};

use crate::orchestrator::context::{ConnectedLocationInfo, ContextBuilder, PresentNpcInfo};
use crate::orchestrator::prompts::QWEN_SYSTEM_PROMPT;
use crate::state::AppState;

#[derive(Clone)]
pub struct TurnOrchestrator {
    state: AppState,
}

impl TurnOrchestrator {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub async fn run_turn(
        &self,
        campaign_id: Uuid,
        player_input: String,
        tx: mpsc::Sender<TurnStreamEvent>,
        cancel_token: CancellationToken,
    ) {
        info!(campaign_id = %campaign_id, "Starting turn orchestration");

        // 1. Pre-load context from DB or fallbacks
        let (campaign_title, player_name, turn_index, location, connected_locations, npcs, recent_turns) =
            if let Some(ref pool) = self.state.db {
                let campaign = match janus_db::campaigns::get_by_id(pool, campaign_id).await {
                    Ok(Some(c)) => c,
                    Ok(None) => {
                        let _ = tx
                            .send(TurnStreamEvent::Error {
                                code: "campaign_not_found".to_string(),
                                message: format!("Campaign {} not found", campaign_id),
                                retryable: false,
                            })
                            .await;
                        return;
                    }
                    Err(e) => {
                        let _ = tx
                            .send(TurnStreamEvent::Error {
                                code: "database_error".to_string(),
                                message: format!("Failed to load campaign: {}", e),
                                retryable: true,
                            })
                            .await;
                        return;
                    }
                };

                let turn_index = campaign.turn_count + 1;
                let title = campaign.title.clone();
                let player_name = campaign.player_name.clone();

                let mut current_loc = None;
                let mut connected_locs = Vec::new();
                let mut npcs_info = Vec::new();

                if let Some(loc_id) = campaign.current_location_id {
                    match janus_db::locations::get_by_id(pool, loc_id).await {
                        Ok(Some(loc)) => {
                            // Connected edges
                            if let Ok(edges) =
                                janus_db::location_edges::list_connected_edges(pool, loc_id).await
                            {
                                for edge in edges {
                                    let neighbor_id = if edge.source_location_id == loc_id {
                                        edge.target_location_id
                                    } else {
                                        edge.source_location_id
                                    };

                                    let neighbor_name =
                                        match janus_db::locations::get_by_id(pool, neighbor_id).await
                                        {
                                            Ok(Some(n)) => n.name,
                                            _ => "Lieu inconnu".to_string(),
                                        };

                                    connected_locs.push(ConnectedLocationInfo {
                                        location_id: neighbor_id,
                                        name: neighbor_name,
                                        is_locked: edge.is_locked,
                                        lock_reason: edge.lock_reason,
                                    });
                                }
                            }

                            // NPCs present
                            if let Ok(loc_npcs) =
                                janus_db::npcs::list_by_location(pool, loc_id).await
                            {
                                for npc in loc_npcs {
                                    let rel =
                                        janus_db::npcs::get_relationship(pool, npc.id).await.ok().flatten();
                                    npcs_info.push(PresentNpcInfo {
                                        npc,
                                        relationship: rel,
                                    });
                                }
                            }

                            current_loc = Some(loc);
                        }
                        Ok(None) => {}
                        Err(e) => warn!("Failed to load current location: {}", e),
                    }
                }

                // Recent turns
                let turns = janus_db::turns::list_by_campaign(pool, campaign_id, 3)
                    .await
                    .unwrap_or_default();

                (
                    title,
                    player_name,
                    turn_index,
                    current_loc,
                    connected_locs,
                    npcs_info,
                    turns,
                )
            } else {
                // Fallback in-memory defaults for tests without live DB
                (
                    "Campagne de Test".to_string(),
                    "Aventurier".to_string(),
                    1,
                    None,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )
            };

        if cancel_token.is_cancelled() {
            warn!("Turn cancelled before start");
            return;
        }

        // 2. Emit turn_start
        let turn_id = Uuid::new_v4();
        if tx
            .send(TurnStreamEvent::TurnStart {
                turn_id,
                campaign_id,
                turn_index,
            })
            .await
            .is_err()
        {
            return;
        }

        // 3. Emit mj_thinking (arbitrating)
        if tx
            .send(TurnStreamEvent::MjThinking {
                status: "arbitrating".to_string(),
                summary: Some(
                    "Arbitrage logique des intentions et consultation du monde".to_string(),
                ),
            })
            .await
            .is_err()
        {
            return;
        }

        // 4. Build MJ TurnPrompt and call complete_turn_arbitration
        let turn_prompt = ContextBuilder::build_turn_prompt(
            &campaign_title,
            &player_name,
            turn_index,
            location.as_ref(),
            &connected_locations,
            &npcs,
            &recent_turns,
            &player_input,
        );

        let mj_response = tokio::select! {
            _ = cancel_token.cancelled() => {
                info!("Turn cancelled during arbitration");
                return;
            }
            res = self.state.llm.complete_turn_arbitration(&turn_prompt) => {
                match res {
                    Ok(resp) => resp,
                    Err(err) => {
                        error!("LLM arbitration failed: {}", err);
                        let _ = tx.send(TurnStreamEvent::Error {
                            code: "arbitration_failed".to_string(),
                            message: err.to_string(),
                            retryable: true,
                        }).await;
                        return;
                    }
                }
            }
        };

        if cancel_token.is_cancelled() {
            return;
        }

        // 5. Execute MCP tools & emit mutations
        let mut executed_mutations: Vec<StateMutation> = Vec::new();
        if !mj_response.tool_calls.is_empty() {
            if tx
                .send(TurnStreamEvent::MjThinking {
                    status: "calling_tools".to_string(),
                    summary: Some(format!(
                        "Exécution de {} action(s) sur l'état du monde",
                        mj_response.tool_calls.len()
                    )),
                })
                .await
                .is_err()
            {
                return;
            }

            for tool_call in &mj_response.tool_calls {
                if cancel_token.is_cancelled() {
                    return;
                }

                if let Some(ref pool) = self.state.db {
                    let mcp_call = janus_mcp::ToolCall {
                        name: tool_call.name.clone(),
                        arguments: tool_call.arguments.clone(),
                    };

                    match janus_mcp::McpExecutor::execute_tool(
                        pool,
                        campaign_id,
                        turn_index,
                        &mcp_call,
                    )
                    .await
                    {
                        Ok(result) => {
                            debug!(tool = %tool_call.name, success = result.success, "Executed MCP tool");
                            if let Some(mutation) = result.mutation {
                                if tx
                                    .send(TurnStreamEvent::StateMutation(mutation.clone()))
                                    .await
                                    .is_err()
                                {
                                    return;
                                }
                                executed_mutations.push(mutation);
                            }
                        }
                        Err(err) => {
                            warn!(tool = %tool_call.name, error = %err, "MCP tool execution warning");
                        }
                    }
                } else {
                    // Simulated mutations for mock testing without DB
                    if let Some(mut_event) = simulate_mock_mutation(tool_call) {
                        if tx
                            .send(TurnStreamEvent::StateMutation(mut_event.clone()))
                            .await
                            .is_err()
                        {
                            return;
                        }
                        executed_mutations.push(mut_event);
                    }
                }
            }
        }

        if cancel_token.is_cancelled() {
            return;
        }

        // 6. Emit mj_thinking (narrating)
        if tx
            .send(TurnStreamEvent::MjThinking {
                status: "narrating".to_string(),
                summary: Some("Rédaction littéraire immersive par La Plume".to_string()),
            })
            .await
            .is_err()
        {
            return;
        }

        // 7. Stream narration from Qwen
        let director_briefing = DirectorBriefing {
            system_prompt: QWEN_SYSTEM_PROMPT.to_string(),
            briefing_instructions: mj_response.director_briefing.clone(),
            context: serde_json::json!({
                "player_input": player_input,
                "location": location.as_ref().map(|l| l.name.clone()),
                "mutations": executed_mutations,
                "turn_index": turn_index,
            }),
        };

        let mut narration_stream = tokio::select! {
            _ = cancel_token.cancelled() => {
                info!("Turn cancelled before narration streaming");
                return;
            }
            res = self.state.llm.stream_narration(&director_briefing) => {
                match res {
                    Ok(s) => s,
                    Err(err) => {
                        error!("Narration streaming initiation failed: {}", err);
                        let _ = tx.send(TurnStreamEvent::Error {
                            code: "narration_init_failed".to_string(),
                            message: err.to_string(),
                            retryable: true,
                        }).await;
                        return;
                    }
                }
            }
        };

        let mut final_narration = String::new();
        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    info!("Turn cancelled during narration stream");
                    return;
                }
                item = narration_stream.next() => {
                    match item {
                        Some(Ok(chunk)) => {
                            final_narration.push_str(&chunk);
                            if tx.send(TurnStreamEvent::NarrationChunk { chunk }).await.is_err() {
                                warn!("SSE receiver dropped by client, aborting turn");
                                return;
                            }
                        }
                        Some(Err(err)) => {
                            error!("Error chunk in narration stream: {}", err);
                            let _ = tx.send(TurnStreamEvent::Error {
                                code: "narration_stream_error".to_string(),
                                message: err.to_string(),
                                retryable: true,
                            }).await;
                            return;
                        }
                        None => break,
                    }
                }
            }
        }

        // 8. Commit and persist turn in PostgreSQL (if DB is active)
        let mut final_location_id = location.as_ref().map(|l| l.id);

        if let Some(ref pool) = self.state.db {
            // Increment campaign turn count
            if let Err(e) = janus_db::campaigns::increment_turn_count(pool, campaign_id).await {
                error!("Failed to increment turn count in DB: {}", e);
            }

            // Save turn
            let new_turn = janus_db::turns::NewTurn {
                id: Some(turn_id),
                campaign_id,
                turn_number: turn_index,
                player_input: player_input.clone(),
                mj_reasoning: Some(mj_response.reasoning.clone()),
                mj_briefing: Some(mj_response.director_briefing.clone()),
                final_narration: final_narration.clone(),
                metadata: Some(serde_json::json!({
                    "tool_calls": mj_response.tool_calls,
                    "mutations": executed_mutations,
                })),
            };

            if let Err(e) = janus_db::turns::create(pool, &new_turn).await {
                error!("Failed to persist turn in DB: {}", e);
            }

            // Reload campaign to get updated current_location_id (in case of movement)
            if let Ok(Some(camp)) = janus_db::campaigns::get_by_id(pool, campaign_id).await {
                final_location_id = camp.current_location_id;
            }
        }

        // 9. Emit turn_complete
        let tool_calls_val = serde_json::to_value(&mj_response.tool_calls).ok();
        let _ = tx
            .send(TurnStreamEvent::TurnComplete {
                turn_id,
                current_location_id: final_location_id,
                turn_summary: mj_response.reasoning.clone(),
                mj_reasoning: Some(mj_response.reasoning),
                mj_briefing: Some(mj_response.director_briefing),
                tool_calls: tool_calls_val,
            })
            .await;

        info!(turn_id = %turn_id, turn_index = turn_index, "Turn orchestration completed successfully");
    }
}

fn simulate_mock_mutation(tool_call: &ToolCall) -> Option<StateMutation> {
    match tool_call.name.as_str() {
        "update_npc_relation" => {
            let args: janus_mcp::UpdateNpcRelationArgs =
                serde_json::from_value(tool_call.arguments.clone()).ok()?;
            Some(StateMutation::RelationshipUpdate {
                npc_id: args.npc_id,
                npc_name: "Elena".to_string(),
                affinity: args.delta_affinity.unwrap_or(10),
                trust: args.delta_trust.unwrap_or(0),
                mood: args.mood.unwrap_or_else(|| "bienveillante".to_string()),
                delta_affinity: args.delta_affinity,
                delta_trust: args.delta_trust,
                reason: args.reason,
            })
        }
        "move_to_location" => {
            let args: janus_mcp::MoveToLocationArgs =
                serde_json::from_value(tool_call.arguments.clone()).ok()?;
            Some(StateMutation::LocationChange {
                location_id: args.target_location_id,
                location_name: "Nouveau Lieu".to_string(),
                narration_hint: args.movement_narration_hint,
            })
        }
        "log_event" => {
            let args: janus_mcp::LogEventArgs =
                serde_json::from_value(tool_call.arguments.clone()).ok()?;
            Some(StateMutation::EventLogged {
                event_id: Uuid::new_v4(),
                summary: args.summary,
                significance: args.significance.unwrap_or_else(|| "notable".to_string()),
            })
        }
        _ => None,
    }
}
