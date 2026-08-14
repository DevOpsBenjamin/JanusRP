# ADR 0006 : Stratégie d'Indexation et RAG pgvector pour la Mémoire à Long Terme (Souvenirs PNJ & Consolidation Narrative)

## Statut
Accepté

## Date
2026-08-14

## Contexte
Dans un sandbox de roleplay narratif persistant, les modèles de langage (LLM) sont confrontés à deux écueils majeurs au fil des tours de jeu :
1. **L'oubli des antécédents et des promesses narratives** : Dès que l'historique dépasse la fenêtre de contexte utile, le MJ et la Plume oublient les faits marquants, trahisons, secrets partagés ou interactions passées avec les PNJ.
2. **La saturation et la pollution du contexte** : Injecter l'intégralité des logs de tous les tours précédents détériore la qualité du raisonnement de Muse Glimmer, augmente drastiquement la latence et génère des coûts de calcul prohibitifs.

Il est donc nécessaire de définir une stratégie d'indexation vectorielle (pgvector) et de RAG (Retrieval-Augmented Generation) sémantique, couplée à un cycle de vie d'extraction mémorielle pour les PNJ et à une consolidation périodique des chapitres.

## Décision

### 1. Abstraction du Fournisseur d'Embeddings (`EmbeddingProvider`)
Le Backend Rust isole la génération des vecteurs d'embeddings derrière un trait asynchrone découplé :

```rust
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError>;
    fn dimension(&self) -> usize;
}
```

- **Adaptateur par défaut (HTTP OpenAI-compatible)** : Client HTTP `reqwest` interrogeant `POST /v1/embeddings` (ex: OpenAI `text-embedding-3-small`, ou endpoint local vLLM / TEI / Ollama avec `bge-m3` ou `bge-multilingual`) en **1536 dimensions** (dimension alignée avec le schéma SQLx `vector(1536)` et l'index HNSW `vector_cosine_ops`).
- **Adaptateur Local In-Process (`FastEmbedAdapter`)** : Moteur local ONNX (`fastembed-rs`) compilable via feature flag pour un fonctionnement 100% hors-ligne.

### 2. Cycle de Vie Hybride des Souvenirs & Extraction Mémorielle
L'extraction et la persistance des souvenirs reposent sur une architecture en deux temps :
- **Enregistrement Synchrone des Faits Saillants (Au cours du tour)** :
  - Muse Glimmer émet les événements majeurs via l'outil MCP existant `log_event(summary, importance)` ou un outil dédié `record_npc_memory(npc_id, memory_text, emotional_impact, importance)`.
  - Ces enregistrements sont immédiatement insérés dans la transaction SQLx du tour.
- **Worker Asynchrone Tokio d'Indexation & Déduction (Post-tour)** :
  - Dès la finalisation du tour (`turn_complete` émis sur le SSE), un worker d'arrière-plan Tokio prend en charge le calcul des embeddings vectoriels et leur mise à jour dans `narrative_events` et la table dédiée `npc_memories`.
  - Si nécessaire, un mini-prompt d'extraction rapide synthétise les impressions individuelles des PNJ présents dans la scène sans jamais ralentir le streaming de la prose destinée au joueur.

### 3. Restitution RAG Hybride (*Push + Pull*)
Pour éliminer les allers-retours superflus tout en offrant une capacité d'investigation illimitée :
- **Pré-injection Automatique (*Push Top-3*)** :
  - Au début de chaque tour, l'orchestrateur calcule l'embedding de l'intention du joueur et de la situation actuelle.
  - Une requête SQL pgvector sélectionne les 3 antécédents les plus pertinents avec similarité cosinus $\ge 0.70$, pondérés par un score d'atténuation temporelle :
    $$\text{Score} = \text{CosineSimilarity}(\vec{q}, \vec{v}) \times (1.0 + 0.1 \times \text{Importance}) \times e^{-\lambda \cdot \Delta \text{tours}}$$
  - Ces 3 faits sont injectés directement dans le bloc contextuel initial de Muse Glimmer (`[Antécédents et Souvenirs Pertinents]`).
- **Outil MCP d'Investigation Mémorielle (*Pull Search*)** :
  - Muse Glimmer dispose de l'outil MCP `search_memories(query: String, npc_id: Option<UUID>, limit: Option<u32>)` lui permettant d'interroger activement la base vectorielle si le joueur fait référence à un événement ancien obscur.

### 4. Consolidation Narrative & Résumés Glissants de Chapitre
Pour conserver la cohérence globale à l'échelle de dizaines de tours :
- **Résumés de Chapitre Glissants** : Tous les 10 à 15 tours (ou lors d'une transition spatiale vers une nouvelle région), une tâche asynchrone condense les événements récents en un paragraphe de synthèse stocké dans `campaigns.metadata -> current_chapter_summary`.
- **Rétention Vectorielle Intelligente** : Les événements élémentaires conservent leur vecteur mais voient leur priorité de rappel atténuée par le decay factor, laissant la priorité aux faits marquants à haute importance ($\text{Importance} \ge 8$).

## Conséquences

### Positives
- **Zéro Impact sur la Latence Utilisateur** : La vectorisation et l'extraction mémorielle détaillée s'exécutent en arrière-plan sans bloquer le flux SSE.
- **Rappel Précis et Organique** : Les PNJ se souviennent naturellement des attitudes passées du PJ grâce à la combinaison pré-injection Top-3 + recherche sémantique active.
- **Pérennité du Contexte** : Les résumés de chapitre glissants évitent la dérive narrative sur les longues campagnes sans dépasser le budget de tokens de Muse Glimmer.

### Négatives / Risques
- Nécessite de calibrer le seuil de similarité cosinus ($\ge 0.70$) et le paramètre d'atténuation $\lambda$ pour éviter les faux positifs mémoriels.
