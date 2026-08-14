# ADR 0007 : Feuille de Route en Tranches Verticales (Vertical Slices Roadmap) et Plan d'Implémentation

## Statut
Accepté

## Date
2026-08-14

## Contexte
À l'issue de la phase de cadrage architectural (ADR-0001 à ADR-0006 : Contrats MCP, Schéma PostgreSQL/SQLx, Pipeline SSE Axum, UI React & DSL narratif, Éditeur de Monde, RAG pgvector), tous les piliers techniques et contrats d'interface de JanusRP sont spécifiés.

Pour passer de la conception à la réalisation logicielle sans s'enliser dans un développement en couches monolithiques déconnectées ("backend first" puis "frontend first"), il est essentiel de découper le projet en **tranches verticales fonctionnelles (Vertical Slices)**. Chaque tranche doit traverser l'ensemble de la pile (Base de données -> Backend Rust / MCP -> LLM -> Frontend React) et produire un incrément immédiatement jouable, testable et démontrable.

Il convient également de fixer l'organisation physique du code (Monorepo Cargo Workspace), la stratégie de test sans dépendance GPU obligatoire, et le scénario de référence d'amorce (*Seed Dataset*).

## Décision

### 1. Organisation du Dépôt (Cargo Workspace & Frontend Dédié)
Le projet adopte une structure Monorepo combinant un workspace Cargo modulaire pour le backend Rust et un package Vite/TypeScript pour le frontend :

```text
/
├── crates/
│   ├── core/         # Types de domaine (Campaign, Location, NPC, Turn), événements SSE, DSL narratif
│   ├── db/           # Schéma PostgreSQL 16 + pgvector, migrations SQLx, modèles et requêtes typées
│   ├── mcp/          # Serveur MCP in-process, catalogue d'outils de tour et exécution transactionnelle
│   ├── llm/          # Trait LlmClient, implémentation HttpLlmClient (vLLM/Aphrodite/Ollama) et MockLlmClient
│   └── server/       # Application web Axum, endpoints REST & SSE, orchestration Tokio (TurnOrchestrator)
├── frontend/         # Application React 18, Vite, TypeScript, ReactFlow, Zustand, TailwindCSS
├── docker/           # Configuration Docker Compose (PostgreSQL 16 + pgvector)
├── docs/             # Documentation d'architecture (ADRs) et guides agents
├── CONTEXT.md        # Glossaire du langage ubiquitaire du domaine
└── Cargo.toml        # Racine du Workspace Cargo Rust
```

### 2. Découpage en 4 Tranches Verticales (Vertical Slices)

#### Slice 1 : MVP Boucle Jouable (*Playable Core Loop*) — Priorité Absolue
* **Objectif** : Obtenir la première boucle de jeu interactive complète de bout en bout.
* **Périmètre technique & fonctionnel** :
  - Déploiement PostgreSQL 16 via Docker Compose et migrations SQLx initiales.
  - Injection du scénario d'amorce (*Seed Dataset* : *"Les Brumes de Val-Corbeau"*).
  - Implémentation du trait `LlmClient` avec `HttpLlmClient` et `MockLlmClient` (fixtures de test).
  - Serveur MCP in-process avec les 5 outils nominaux (`get_location_context`, `inspect_npc_details`, `update_npc_relation`, `move_to_location`, `log_event`).
  - Pipeline de tour Axum `POST /api/campaigns/:id/turns` avec verrous de campagne et flux SSE multiplexé (`turn_start`, `mj_thinking`, `state_mutation`, `narration_chunk`, `turn_complete`, `error`).
  - Console React 3 volets :
    - Volet gauche : Graphe ReactFlow passif avec centrage automatique sur le lieu actif du PJ.
    - Volet central : Flux narratif en streaming avec parseur tolérant `StreamingRPParser` et rendu stylisé du DSL (`<dialogue>`, `<thought>`, `<sensory>`, `<document>`).
    - Volet droit : Inspecteur PNJ avec jauges relationnelles (Affinité, Confiance, Humeur) animées en temps réel.
* **Critère d'acceptation (DoD)** : Un joueur saisit une action textuelle libre dans l'auberge ; Muse Glimmer arbitre l'action via MCP (mutation d'affinité ou déplacement) ; Qwen diffuse la prose en streaming ; la transaction SQLx est validée et l'interface React réagit instantanément sans rechargement.

#### Slice 2 : Éditeur de Monde & Roster PNJ (*Worldbuilding Studio*)
* **Objectif** : Permettre la création, l'agencement cartographique et la configuration des campagnes et PNJ directement dans l'interface sans accès direct à la base de données.
* **Périmètre technique & fonctionnel** :
  - Route `/campaigns/:id/editor` avec palette de nœuds, roster de PNJ et canvas ReactFlow interactif.
  - Manipulation visuelle des nœuds (`locations`), liaisons navigables (`location_edges`) et contraintes de passage (`is_locked`).
  - Drag & drop des fiches PNJ sur les nœuds de lieux pour assignation géographique.
  - Synchronisation bidirectionnelle REST avec débounce 300 ms (`POST/PUT/DELETE /api/campaigns/:id/locations`, `/edges`, `/npcs`).
  - Module d'assistance IA pour la génération procédurale JSON de descriptions de lieux et de profils psychologiques de PNJ.

#### Slice 3 : Mémoire Vectorielle & RAG Hybride (*Episodic Semantic Memory*)
* **Objectif** : Doter le MJ et les PNJ d'une mémoire épisodique à long terme pour restituer les faits marquants et dialogues anciens sans perte de contexte.
* **Périmètre technique & fonctionnel** :
  - Intégration de l'index HNSW `pgvector` (`vector(1536)`) et abstraction `EmbeddingProvider` (API OpenAI / local FastEmbed ONNX).
  - Worker asynchrone Tokio d'indexation post-tour (`npc_memories` et `narrative_events`).
  - Restitution RAG hybride : pré-injection Top-3 cosine $\ge 0.70$ avec pondération temporelle + outil MCP `search_memories`.
  - Synthèses narratives glissantes de chapitres toutes les 10-15 tours dans les métadonnées de campagne.

#### Slice 4 : Mécaniques Avancées & Qualité (*Advanced Mechanics & Guardrails*)
* **Objectif** : Enrichir la palette d'arbitrage matériel et sécuriser la conformité narrative.
* **Périmètre technique & fonctionnel** :
  - Gestion dynamique d'inventaires, contraintes de poids/encombrement et transferts d'objets via outils MCP dédiés.
  - Résolution narrative de phases de confrontation / combat tactique sans calculs mathématiques rigides.
  - Filtre de validation "LLM-as-a-Judge" (relecture de la prose générée par Muse Glimmer pour vérifier l'absence d'hallucinations matérielles ou de contradictions avec l'état du monde, activable par feature flag).

### 3. Stratégie de Test et Mocking LLM
Pour garantir des tests automatisés rapides, déterministes et exécutables en CI/CD sans cluster GPU :
- **Trait `LlmClient`** :
  ```rust
  #[async_trait]
  pub trait LlmClient: Send + Sync {
      async fn complete_turn_arbitration(&self, prompt: &TurnPrompt) -> Result<MjArbitrationResponse, LlmError>;
      async fn stream_narration(&self, briefing: &DirectorBriefing) -> Result<Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>, LlmError>;
  }
  ```
- **`MockLlmClient`** : Rejoue des fixtures JSON pré-enregistrées (appels d'outils MCP ATEM/OpenAI simulés pour Muse Glimmer et flux de tokens Markdown/XML pré-découpés pour Qwen).
- **Tests d'intégration de bout en bout** : Validation complète de la route SSE Axum et du cycle de vie de la transaction SQLx sous `cargo test` en moins d'une seconde.

### 4. Jeu de Données de Référence (*Seed Scenario*)
Le script de migration initial injecte la campagne d'amorce suivante pour validation immédiate :
- **Campagne** : *"Les Brumes de Val-Corbeau"*
- **Lieux** :
  - *La Salle Commune* (Lieu de départ du PJ, chaleureux, feu de cheminée, coordonnées 2D: [0, 0]).
  - *L'Arrière-Cour* (Lieu sombre, issue de secours, coordonnées 2D: [250, 0], relié à la Salle Commune).
  - *La Chambre Haute* (Lieu privé, coordonnées 2D: [0, -200], arête verrouillée `is_locked: true`).
- **PNJ** :
  - *Elena la Tavernier* (Salle Commune, Affinité 0, Confiance 20, Humeur "bienveillante mais vigilante", Secret : recèle des vivres pour des fugitifs).
  - *Gaston le Rôdeur* (Arrière-Cour, Affinité -20, Confiance 0, Humeur "méfiant", Secret : surveille les allées et venues pour une guilde rivale).

## Conséquences

### Positives
- **Découplage et vélocité** : La structure Cargo Workspace sépare clairement les contrats de domaine, les requêtes DB, l'outillage MCP et les clients LLM.
- **Délivrance rapide de valeur** : La Slice 1 permet d'avoir un jeu opérationnel et immersif dès la première itération de développement.
- **Validation CI/CD sans friction** : Le `MockLlmClient` autorise des tests automatisés rigoureux sans dépendance externe.

### Négatives / Risques
- Nécessite de maintenir les fixtures du `MockLlmClient` alignées avec l'évolution des prompts système de Muse Glimmer et Qwen.
