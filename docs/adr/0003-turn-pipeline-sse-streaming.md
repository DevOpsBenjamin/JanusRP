# ADR 0003 : Pipeline Asynchrone de Tour, Gestion de la Concurrence et Streaming SSE

## Statut
Accepté

## Date
2026-08-14

## Contexte
JanusRP orchestre une séquence complexe et asynchrone lors de chaque interaction joueur :
1. Réception de l'intention brute du joueur.
2. Pré-chargement du contexte spatial/social immédiat.
3. Consultation et arbitrage logique par Meta Muse Glimmer 30B (MJ) via des appels d'outils MCP in-process.
4. Mutation de l'état du monde (déplacement topologique, mise à jour des jauges relationnelles, journalisation d'événements).
5. Rédaction littéraire sensorielle et dialogues par Qwen 3.8 (La Plume).
6. Persistance atomique dans PostgreSQL et restitution en direct vers l'interface utilisateur.

Deux contraintes critiques devaient être résolues :
- **L'expérience utilisateur en streaming et multiplexage d'état** : L'interface doit afficher la progression du MJ, répercuter instantanément les mutations visuelles sur le graphe de la carte (ReactFlow) et streamer la prose de la Plume token par token sans attendre la fin du calcul.
- **La robustesse de concurrence et l'intégrité transactionnelle** : Éviter les états corrompus en cas d'actions concurrentes sur une même campagne ou en cas de coupure de stream / crash LLM au milieu d'un tour.

## Décision

1. **Transport HTTP et Endpoint SSE Unique** :
   - L'initiation d'un tour s'effectue via `POST /api/campaigns/:id/turns` avec `Accept: text/event-stream`.
   - La réponse HTTP est un flux SSE (`axum::response::sse::Sse`) multiplexé typé tout au long de la boucle de tour.

2. **Protocole d'Événements SSE Multiplexés** :
   - `turn_start` : `{ turn_id, campaign_id, turn_index }` (initialisation du tour).
   - `mj_thinking` : `{ status: "arbitrating" | "calling_tools", summary?: string }` (télémétrie de l'arbitre).
   - `state_mutation` : `{ type: "location_change" | "relationship_update" | "event_logged", payload: { ... } }` (permet au frontend et au graphe ReactFlow de muter en direct).
   - `narration_chunk` : `{ chunk: string }` (streaming token-par-token de la prose de Qwen).
   - `turn_complete` : `{ turn_id, current_location, updated_npcs: [...], turn_summary: string }` (clôture du tour).
   - `error` : `{ code: string, message: string, retryable: bool }` (signalement d'erreur).

3. **Gestion de la Concurrence et Annulation Coopérative** :
   - **Verrouillage par campagne** : Un verrou en mémoire (`tokio::sync::Mutex` par campagne dans l'état partagé `AppState`) garantit la sérialisation stricte des tours. Toute tentative d'initier un tour alors qu'un tour est en cours retourne immédiatement un code `HTTP 409 Conflict`.
   - **Verrouillage UI** : L'interface utilisateur désactive les contrôles de saisie dès l'envoi de l'intention et jusqu'à la réception de `turn_complete` ou `error`.
   - **Annulation par CancellationToken** : Un `tokio_util::sync::CancellationToken` est attaché à la connexion client. Si le client ferme le flux SSE, la tâche de génération de Qwen et les calculs sont immédiatement avortés pour libérer les ressources d'inférence.

4. **Atomicité Transactionnelle SQLx** :
   - L'orchestrateur de tour ouvre une transaction SQLx au début du traitement.
   - Les mutations MCP sont exécutées dans cette transaction active.
   - Le commit final de la transaction (persistance de l'état modifié, insertion dans `turns` et `narrative_events`) intervient uniquement après la réussite de la génération du récit de Qwen.
   - En cas d'erreur ou d'annulation avant la complétion, la transaction fait l'objet d'un rollback complet, préservant l'intégrité de l'univers.

5. **Architecture Modulaire Tokio** :
   - Un `TurnOrchestrator` gère le cycle de vie et communique avec le handler Axum via un `tokio::sync::mpsc::channel<TurnStreamEvent>`.
   - Le handler Axum convertit le récepteur en `tokio_stream::wrappers::ReceiverStream` adapté pour `axum::response::sse::Event`.
   - Découpage interne : `ContextBuilder` (pré-injection), `GlimmerClient` (arbitrage MJ + parser ATEM), `McpExecutor` (exécution in-process), `QwenClient` (client streaming Aphrodite/vLLM), et `PersistenceService` (transactions SQLx).

## Conséquences

### Positives
- **Réactivité perçue maximale** : Le joueur visualise la réflexion du MJ, voit le graphe de la carte s'actualiser en temps réel lors d'un déplacement, et lit le texte en cours de génération sans latence de blocage.
- **Intégrité absolue des données** : Zéro risque de désynchronisation entre la base SQL et l'état narratif en cas d'erreur de la Plume ou de déconnexion.
- **Protection contre la concurrence** : Isolation garantie par campagne et verrouillage UI ergonomique.

### Négatives / Risques
- Nécessite la gestion propre du cycle de vie des verrous en mémoire (`AppState`) et du nettoyage des canaux Tokio en cas de crash inattendu.
