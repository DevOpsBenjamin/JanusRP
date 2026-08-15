use janus_core::StateMutation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    pub tool_name: String,
    pub success: bool,
    pub result: serde_json::Value,
    pub mutation: Option<StateMutation>,
    pub error: Option<String>,
}

impl ToolExecutionResult {
    pub fn success(tool_name: impl Into<String>, result: serde_json::Value, mutation: Option<StateMutation>) -> Self {
        Self {
            tool_name: tool_name.into(),
            success: true,
            result,
            mutation,
            error: None,
        }
    }

    pub fn failure(tool_name: impl Into<String>, error_message: impl Into<String>) -> Self {
        let msg = error_message.into();
        Self {
            tool_name: tool_name.into(),
            success: false,
            result: serde_json::json!({ "error": msg }),
            mutation: None,
            error: Some(msg),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLocationContextArgs {
    pub location_id: Option<Uuid>,
    pub include_secrets: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectNpcDetailsArgs {
    pub npc_id: Uuid,
    pub query_focus: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNpcRelationArgs {
    pub npc_id: Uuid,
    pub delta_affinity: Option<i32>,
    pub delta_trust: Option<i32>,
    pub mood: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveToLocationArgs {
    pub target_location_id: Uuid,
    pub force: Option<bool>,
    pub movement_narration_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEventArgs {
    pub summary: String,
    pub significance: Option<String>,
    pub involved_npc_ids: Option<Vec<Uuid>>,
    pub location_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
}

pub fn get_turn_tools_schema() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "get_location_context".to_string(),
            description: "Consultation de la topologie, ambiance et PNJ d'un lieu.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "location_id": { "type": "string", "description": "ID UUID du lieu (optionnel, défaut: lieu actuel du joueur)" },
                    "include_secrets": { "type": "boolean", "description": "Inclure les secrets découverts" }
                }
            }),
        },
        ToolDefinition {
            name: "inspect_npc_details".to_string(),
            description: "Consultation approfondie de la psychologie, secrets et souvenirs d'un PNJ.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "required": ["npc_id"],
                "properties": {
                    "npc_id": { "type": "string", "description": "ID UUID du PNJ" },
                    "query_focus": { "type": "string", "description": "Focalisation spécifique (ex: secrets, background)" }
                }
            }),
        },
        ToolDefinition {
            name: "update_npc_relation".to_string(),
            description: "Mutation par deltas relatifs calculés et bornés [-100..100] des jauges relationnelles d'un PNJ.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "required": ["npc_id", "reason"],
                "properties": {
                    "npc_id": { "type": "string", "description": "ID UUID du PNJ" },
                    "delta_affinity": { "type": "integer", "minimum": -50, "maximum": 50, "description": "Variation d'affinité" },
                    "delta_trust": { "type": "integer", "minimum": -50, "maximum": 50, "description": "Variation de confiance" },
                    "mood": { "type": "string", "description": "Nouvelle humeur du PNJ" },
                    "reason": { "type": "string", "description": "Justification narrative du changement relationnel" }
                }
            }),
        },
        ToolDefinition {
            name: "move_to_location".to_string(),
            description: "Déplacement du joueur vers un lieu adjacent (ou forcé si action magique/rupture).".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "required": ["target_location_id"],
                "properties": {
                    "target_location_id": { "type": "string", "description": "ID UUID du lieu de destination" },
                    "force": { "type": "boolean", "description": "Forcer le déplacement sans vérifier la contrainte topologique d'adjacence" },
                    "movement_narration_hint": { "type": "string", "description": "Indication pour la narration du déplacement" }
                }
            }),
        },
        ToolDefinition {
            name: "log_event".to_string(),
            description: "Journalisation d'un fait marquant immuable en base de données pour la mémoire épisodique.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "required": ["summary"],
                "properties": {
                    "summary": { "type": "string", "description": "Résumé factuel de l'événement" },
                    "significance": { "type": "string", "enum": ["minor", "notable", "major", "critical"], "default": "notable" },
                    "involved_npc_ids": { "type": "array", "items": { "type": "string" } },
                    "location_id": { "type": "string" },
                    "tags": { "type": "array", "items": { "type": "string" } }
                }
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_tools_schema_count() {
        let tools = get_turn_tools_schema();
        assert_eq!(tools.len(), 5);
        let names: Vec<String> = tools.into_iter().map(|t| t.name).collect();
        assert!(names.contains(&"get_location_context".to_string()));
        assert!(names.contains(&"inspect_npc_details".to_string()));
        assert!(names.contains(&"update_npc_relation".to_string()));
        assert!(names.contains(&"move_to_location".to_string()));
        assert!(names.contains(&"log_event".to_string()));
    }

    #[test]
    fn test_deserialize_args() {
        let loc_args: GetLocationContextArgs = serde_json::from_value(serde_json::json!({
            "include_secrets": true
        })).unwrap();
        assert_eq!(loc_args.include_secrets, Some(true));
        assert_eq!(loc_args.location_id, None);

        let update_args: UpdateNpcRelationArgs = serde_json::from_value(serde_json::json!({
            "npc_id": "00000000-0000-0000-0000-000000000001",
            "delta_affinity": 15,
            "reason": "Test"
        })).unwrap();
        assert_eq!(update_args.delta_affinity, Some(15));
        assert_eq!(update_args.delta_trust, None);
        assert_eq!(update_args.reason, "Test");
    }
}
