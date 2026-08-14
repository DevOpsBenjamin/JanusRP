# JanusRP — Ubiquitous Language & Domain Glossary

## Core Concepts

### Sandbox Narratif
Un environnement de jeu de rôle interactif piloté par l'intention du joueur, où l'histoire et les conséquences émergent dynamiquement sans rails rigides ni systèmes de règles mathématiques contraignants.

### Joueur (PJ - Personnage Joueur)
L'utilisateur humain qui interagit avec le monde en saisissant des intentions d'actions brutes, des dialogues ou des choix narratifs.

### PNJ (Personnage Non-Joueur)
Entité vivante du monde dotée d'une identité, d'un profil psychologique, de relations (affinité, statut émotionnel, rancunes), de secrets et de souvenirs persistants.

### Le Maître du Jeu (MJ / Décisionnaire)
L'agent d'arbitrage logique et contextuel (Meta Muse Glimmer). Il interprète l'intention brute du joueur, interroge l'état du monde, prend les décisions d'arbitrage (succès, échecs, évolutions relationnelles), commande les mutations d'état et rédige le briefing narratif pour la Plume.

### La Plume (Le Narrateur / Incarnateur)
L'agent de rédaction littéraire et stylistique (Qwen). Il transforme les directives narratives et les décisions d'arbitrage du MJ en descriptions immersives, sensorielles et en dialogues de PNJ fidèles à leur ton.

### Boucle de Tour (*Turn Lifecycle*)
La séquence complète déclenchée par une entrée du joueur : réception de l'intention -> consultation de l'état (MCP) -> arbitrage & décision par le MJ -> mutation d'état (MCP) -> briefing narratif -> génération du récit par la Plume -> streaming vers le joueur.

### Outils MCP (*Model Context Protocol*)
L'ensemble des fonctions exposées par le backend pour permettre au MJ d'interroger et de modifier l'état persistant du monde (lieux, PNJ, inventaires, relations, mémoire) de manière structurée et sans hallucination.

### Graphe du Monde (*World Graph*)
La structure relationnelle représentant les lieux (nœuds), leurs liaisons topologiques (arêtes), ainsi que la présence actuelle des PNJ et objets dans ces lieux.

### Jauges Relationnelles (*Relationship Gauges*)
Les métriques numériques bornées (`Affinité`, `Confiance`, sur une échelle de -100 à +100) et qualitatives (`Humeur`) mesurant la disposition psychologique et l'attitude d'un PNJ envers le PJ.

### Événement Narratif Persistant (*Narrative Event Log*)
Fait saillant immuable consigné à l'issue de chaque tour notable, servant d'ancrage chronologique et de base d'indexation pour la mémoire épisodique du monde.

### Briefing Narratif (*Director Briefing*)
Consigne scénaristique et stylistique explicite produite par le MJ (Muse Glimmer) à l'attention exclusive de la Plume (Qwen), définissant le ton, les réactions sensorielles et les dialogues à générer sans réévaluer les règles.

### Campagne (*Campaign / World Instance*)
Instance isolée d'un univers de jeu contenant son propre graphe de lieux, ses PNJ, son historique de tours et son état de progression.

### Topologie Spatiale (*Spatial Topology*)
Réseau de nœuds (`locations`) et d'arêtes (`location_edges`) définissant les chemins navigables, agrémenté de coordonnées 2D pour la restitution cartographique interactive (ReactFlow).

### Mémoire Vectorielle & Indexation RAG (*Semantic Episodic Memory*)
Stockage vectoriel des événements narratifs et souvenirs (`pgvector`, embeddings 1536d) indexé en HNSW pour la restitution contextuelle des antécédents marquants lors des délibérations du MJ.

### Orchestrateur de Tour (*Turn Orchestrator*)
Le moteur asynchrone Rust (Tokio) responsable de la coordination de la boucle de tour : acquisition du verrou de campagne, ouverture de la transaction SQLx, pré-injection du contexte, invocation de Muse Glimmer (MJ) et exécution MCP in-process, streaming de la prose de Qwen (La Plume) et commit atomique.

### Événements de Flux SSE (*SSE Stream Events*)
Protocole de messages multiplexés typés (`turn_start`, `mj_thinking`, `state_mutation`, `narration_chunk`, `turn_complete`, `error`) émis en temps réel sur la connexion HTTP SSE du tour pour synchroniser l'affichage narratif et l'état réactif du monde (graphe ReactFlow, fiches PNJ).

### Balises Sémantiques Narratives (*RP Tags DSL*)
Grammaire de balises de registres narratifs (`<narrative>`, `<dialogue>`, `<thought>`, `<comm>`, `<sensory>`, `<document>`, `<illustration>`) générée par La Plume (Qwen) pour structurer visuellement et dynamiquement la mise en scène du roleplay.

### Parser Streaming Tolérant (*StreamingRPParser*)
Parseur incrémental côté frontend capable de segmenter le flux de tokens en blocs de composants React typés en temps réel, résistant aux balises en cours d'ouverture, aux coupures et aux éléments de texte libre orphelins.

### Nœud de Lieu ReactFlow (*LocationNode*)
Composant cartographique personnalisé représentant un lieu du monde dans ReactFlow avec ses badges d'état (lieu actif du PJ, présence de PNJ, liaisons navigables et centrage caméra dynamique).

### Éditeur de Monde (*World Editor*)
Espace interactif de modélisation cartographique et sociale (`/campaigns/:id/editor`) permettant de construire visuellement le graphe de navigation (ReactFlow), d'agencer les lieux et de configurer l'ensemble des entités de jeu hors session active.

### Roster de PNJ (*Campaign NPC Roster*)
Registre centralisé et catalogue des personnages non-joueurs d'une campagne dans l'éditeur, permettant leur création, la configuration de leurs secrets/jauges et leur assignation géographique par glisser-déposer sur les nœuds de lieux.

### Arête Topologique Éditée (*Location Edge / Spatial Link*)
Liaison navigable orientée ou bidirectionnelle entre deux lieux dans le graphe, caractérisée par ses conditions de franchissement (`is_locked`), sa description de transit et sa connectivité ReactFlow.

### Assistance IA de Création (*AI Content Assistant*)
Module de génération procédurale assistée par LLM produisant des structures JSON pour pré-remplir automatiquement les profils psychologiques de PNJ, les intrigues secrètes, les descriptions immersives de lieux et les dynamiques relationnelles.

### Fournisseur d'Embeddings (*Embedding Provider*)
Composant d'abstraction backend Rust (`EmbeddingProvider`) capable de vectoriser du texte via une API HTTP compatible OpenAI (`/v1/embeddings`, 1536d) ou un moteur ONNX local in-process (`fastembed-rs`) sans coupler l'architecture à un modèle unique.

### Souvenir Individuel de PNJ (*NPC Episodic Memory*)
Trace mémorielle persistante associée à un PNJ spécifique, caractérisée par un texte récapitulatif, une polarité émotionnelle, un coefficient d'importance et son vecteur d'embedding, restituée lors des interactions futures avec ce PNJ.

### Restitution RAG Hybride (*Hybrid Memory Retrieval*)
Mécanisme combinant la pré-injection systématique des antécédents les plus similaires (Top-3 cosinus > 0.70 avec atténuation temporelle) dans le contexte initial de Muse Glimmer et la mise à disposition d'un outil MCP `search_memories` pour les investigations historiques approfondies.

### Consolidation Narrative & Résumé Glissant (*Chapter Memory Consolidation*)
Synthèse périodique (tous les 10 à 15 tours ou lors d'un déplacement d'envergure) enregistrée dans les métadonnées de campagne (`current_chapter_summary`) pour maintenir une vision macroscopique du récit sans saturer le stockage vectoriel d'événements élémentaires.

### Outil de Recherche Mémorielle (*search_memories MCP Tool*)
Outil MCP permettant à Muse Glimmer d'interroger sémantiquement la mémoire de campagne et les souvenirs de PNJ à l'aide d'une requête textuelle et de filtres optionnels par PNJ ou lieu.


