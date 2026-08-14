# ADR 0001 : Contrats d'Outils MCP et Stratégie d'Orchestration pour Muse Glimmer (MJ)

## Statut
Accepté

## Date
2026-08-14

## Contexte
JanusRP sépare strictement l'arbitrage logique du jeu (attribué à Meta Muse Glimmer 30B, le Maître du Jeu) et la narration littéraire (attribuée à Qwen 3.8, la Plume).
Le Maître du Jeu doit être capable de consulter l'état du monde persistant et d'ordonner des mutations d'état sans hallucination via le protocole MCP (Model Context Protocol).

Deux problématiques majeures se posaient :
1. **La latence de la boucle de tour** : Une boucle multi-tours pure où le MJ doit d'abord demander le contexte de lieu via MCP avant de réfléchir puis de muter l'état engendre au minimum 2 à 3 allers-retours LLM (plusieurs secondes de latence avant le début de la génération littéraire).
2. **La cohérence des mutations d'état** : La mise à jour des relations de PNJ (affinité, confiance) et les déplacements doivent être robustes aux erreurs de calcul arithmétique du LLM tout en autorisant des exceptions dramatiques/narratives (téléportation, passage secret).

## Décision

1. **Modèle d'Orchestration Hybride** :
   - Le backend Rust injecte systématiquement le contexte immédiat (lieu actuel, PNJ présents, jauges actives) dans le prompt initial du tour.
   - 90% des tours sont résolus en **un seul appel LLM** pour Muse Glimmer, où le modèle émet ses appels d'outils de mutation (`update_npc_relation`, `move_to_location`, `log_event`) et son briefing narratif.
   - Des outils d'inspection approfondie (`inspect_npc_details`, `get_location_context`) restent disponibles dans le catalogue MCP si le MJ a besoin de fouiller des souvenirs lointains ou des lieux distants.

2. **Catalogue d'Outils MCP et Signatures** :
   - `get_location_context(location_id?: string, include_secrets?: bool)` : Consultation de la topologie, ambiance et PNJ d'un lieu.
   - `inspect_npc_details(npc_id: string, query_focus?: string)` : Consultation approfondie de la psychologie, secrets et souvenirs d'un PNJ.
   - `update_npc_relation(npc_id: string, delta_affinity?: int [-50..50], delta_trust?: int [-50..50], mood?: string, reason: string)` : Mutation par deltas relatifs calculés et bornés [-100..100] par le backend Rust.
   - `move_to_location(target_location_id: string, force?: bool, movement_narration_hint?: string)` : Déplacement avec vérification topologique par défaut et bypass possible via `force: true`.
   - `log_event(summary: string, significance: "minor"|"notable"|"major"|"critical", involved_npc_ids?: string[], location_id?: string, tags?: string[])` : Journalisation d'un fait marquant immuable en base.

3. **Séparation Événement factuel vs Briefing narratif** :
   - `log_event` capture l'enregistrement persistant en base de données.
   - Le briefing pour Qwen est extrait de la sortie textuelle finale de Glimmer (ou bloc structuré dédié), sans polluer la table des événements factuels.

## Conséquences

### Positives
- **Latence minimisée** : Réduction drastique du time-to-first-token pour l'utilisateur.
- **Robustesse arithmétique** : Le backend Rust garantit le clamping et l'intégrité des jauges relationnelles.
- **Flexibilité narrative** : Le flag `force` sur `move_to_location` permet des ruptures topologiques maîtrisées par le MJ sans crash d'intégrité.
- **Séparation claire des préoccupations** : Glimmer arbitre et mute via MCP ; Qwen reçoit un briefing épuré et se concentre sur le style et les dialogues.

### Négatives / Risques
- Le prompt initial envoyé à Glimmer est légèrement plus volumineux en tokens d'entrée du fait de la pré-injection du contexte local.
