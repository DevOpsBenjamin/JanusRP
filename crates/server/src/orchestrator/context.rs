use janus_core::{Location, Npc, NpcRelationship, Turn};
use janus_llm::TurnPrompt;

use super::prompts::MJ_SYSTEM_PROMPT;

pub struct ContextBuilder;

pub struct PresentNpcInfo {
    pub npc: Npc,
    pub relationship: Option<NpcRelationship>,
}

pub struct ConnectedLocationInfo {
    pub location_id: uuid::Uuid,
    pub name: String,
    pub is_locked: bool,
    pub lock_reason: Option<String>,
}

impl ContextBuilder {
    pub fn build_context_summary(
        campaign_title: &str,
        player_name: &str,
        turn_index: i32,
        current_location: Option<&Location>,
        connected_locations: &[ConnectedLocationInfo],
        npcs: &[PresentNpcInfo],
        recent_turns: &[Turn],
    ) -> String {
        let mut out = String::new();

        out.push_str(&format!(
            "### Campagne : {}\nJoueur : {}\nTour actuel : #{}\n\n",
            campaign_title, player_name, turn_index
        ));

        // Location context
        if let Some(loc) = current_location {
            out.push_str(&format!(
                "### Lieu Actuel : {} (id: {})\nDescription : {}\n",
                loc.name, loc.id, loc.description
            ));
            if let Some(ref atm) = loc.atmosphere {
                out.push_str(&format!("Ambiance : {}\n", atm));
            }
            if let Some(ref sec) = loc.secrets {
                out.push_str(&format!("Secrets du lieu (confidentiel MJ) : {}\n", sec));
            }

            if !connected_locations.is_empty() {
                out.push_str("\nPassages et lieux adjacents accessibles :\n");
                for edge in connected_locations {
                    if edge.is_locked {
                        out.push_str(&format!(
                            "- {} (id: {}) [VERROUILLÉ: {}]\n",
                            edge.name,
                            edge.location_id,
                            edge.lock_reason.as_deref().unwrap_or("fermé")
                        ));
                    } else {
                        out.push_str(&format!("- {} (id: {})\n", edge.name, edge.location_id));
                    }
                }
            }
            out.push('\n');
        } else {
            out.push_str("### Lieu Actuel : Inconnu\n\n");
        }

        // NPCs present
        if !npcs.is_empty() {
            out.push_str("### Personnages Non-Joueurs (PNJ) Présents :\n");
            for item in npcs {
                let npc = &item.npc;
                out.push_str(&format!(
                    "- **{}** (id: {})",
                    npc.name, npc.id
                ));
                if let Some(ref title) = npc.title {
                    out.push_str(&format!(", {}", title));
                }
                out.push('\n');

                if let Some(ref bg) = npc.background {
                    out.push_str(&format!("  Background : {}\n", bg));
                }
                if let Some(ref secret) = npc.secret_agenda {
                    out.push_str(&format!("  Agenda secret (confidentiel MJ) : {}\n", secret));
                }

                if let Some(ref rel) = item.relationship {
                    out.push_str(&format!(
                        "  Jauges : Affinité {}/100, Confiance {}/100, Humeur : {}\n",
                        rel.affinity, rel.trust, rel.mood
                    ));
                    if let Some(ref summary) = rel.interaction_summary {
                        out.push_str(&format!("  Historique relationnel : {}\n", summary));
                    }
                } else {
                    out.push_str("  Jauges : Affinité 0/100, Confiance 0/100, Humeur : neutre\n");
                }
            }
            out.push('\n');
        } else {
            out.push_str("### PNJ Présents : Aucun PNJ en vue.\n\n");
        }

        // Recent history
        if !recent_turns.is_empty() {
            out.push_str("### Historique Récent des Tours :\n");
            // Recent turns ordered ascending for chronological readability
            let mut sorted_turns = recent_turns.to_vec();
            sorted_turns.sort_by_key(|t| t.turn_number);

            for t in sorted_turns {
                out.push_str(&format!("* Tour #{}:\n", t.turn_number));
                out.push_str(&format!("  - Joueur : \"{}\"\n", t.player_input));
                if let Some(ref reasoning) = t.mj_reasoning {
                    out.push_str(&format!("  - Arbitrage MJ : {}\n", reasoning));
                }
                let narration_snippet = if t.final_narration.len() > 180 {
                    format!("{}...", &t.final_narration[..180])
                } else {
                    t.final_narration.clone()
                };
                out.push_str(&format!("  - Narration : {}\n", narration_snippet.trim()));
            }
            out.push('\n');
        }

        out
    }

    pub fn build_turn_prompt(
        campaign_title: &str,
        player_name: &str,
        turn_index: i32,
        current_location: Option<&Location>,
        connected_locations: &[ConnectedLocationInfo],
        npcs: &[PresentNpcInfo],
        recent_turns: &[Turn],
        player_input: &str,
    ) -> TurnPrompt {
        let context_summary = Self::build_context_summary(
            campaign_title,
            player_name,
            turn_index,
            current_location,
            connected_locations,
            npcs,
            recent_turns,
        );

        TurnPrompt {
            system_prompt: MJ_SYSTEM_PROMPT.to_string(),
            context_summary,
            player_input: player_input.to_string(),
        }
    }
}
