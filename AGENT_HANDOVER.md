# 📜 AGENT HANDOVER : JanusRP (Version Août 2026)

> **DOCUMENT DE TRANSMISSION OFFICIEL POUR LE PROJET JANUSRP**
> 
> *Ce document synthétise la vision validée, les choix d'architecture technique et le prompt de lancement pour le skill `/wayfinder`.*

---

## 1. 🎯 La Vision Validée : Sandbox Narratif & Roleplay Immersif

**JanusRP** n'est pas un simulateur de jeu de rôle mathématique lourd (pas de lancers de dés D20 obligatoires ni de calculs de PV/mana fastidieux). 

C'est un **Sandbox Narratif & Roleplay Immersif**, conçu pour :
1. **Comprendre l'intention brute du joueur :** Le joueur saisit des actions courtes ou mal formulées (*"je vais lui parler avec un sourire en coin"*). Le système comprend l'intention, évalue les conséquences dans le monde, et génère une scène littéraire riche, sensorielle et incarnée.
2. **Mémoire & État persistant sans hallucinations :** Le monde, la carte, l'état des lieux, les PNJ, leurs relations, secrets et souvenirs sont persistés dans une base de données relationnelle. Le LLM ne stocke pas tout dans son contexte : il consulte et modifie l'état via des outils **MCP (Model Context Protocol)**.
3. **Incarnation & Liberté totale de ton :** Capable de gérer des récits matures, sombres, romantiques ou NSFW sans filtres moralisateurs.
4. **Co-Worldbuilding assisté :** Une interface et un assistant (Muse) pour créer et étendre des univers facilement (graphe de lieux, profils de PNJ, règles d'ambiance).

---

## 2. 🤖 Répartition des Rôles : Les Deux Cerveaux

| Rôle | Modèle / Composant | Responsabilités |
| :--- | :--- | :--- |
| **Le Maître du Jeu / Décisionnaire** | **Meta Muse Glimmer (30B)** | • Reçoit l'intention brute du joueur.<br>• Consulte l'état du monde via les outils MCP.<br>• **Prend toutes les décisions d'arbitrage** (ex: *le compliment réussit, Elena gagne +10 d'affinité*).<br>• Ordonne les mutations d'état au Backend via MCP.<br>• Prépare le briefing narratif précis pour le rédacteur. |
| **La Plume / Le Narrateur** | **Qwen 3.8 (27B local ou Max API)** | • Modèle débridé à très haute qualité littéraire en français.<br>• Transforme le briefing du MJ en descriptions viscérales et dialogues vivants.<br>• Incarne fidèlement les PNJ selon leur profil psychologique. |
| **Le Gardien de l'État & Support** | **Backend Rust + PostgreSQL** | • Stockage persistant passif (carte/graphe, inventaire, relations PNJ, logs).<br>• Expose le serveur MCP pour Muse.<br>• Gère l'API REST / WebSocket / SSE vers le frontend.<br>• N'embarque **aucune logique d'arbitrage hardcodée** : il exécute les ordres de Muse. |

---

## 3. 🛠️ Stack Technique Validée

* **Backend :** **Rust** (`axum` pour le serveur web async, `sqlx` pour PostgreSQL typé, `serde` pour la manipulation JSON/MCP, `tokio`).
* **Base de données :** **PostgreSQL 16+** (avec `pgvector` pour la mémoire RAG à terme), orchestré via **Docker Compose**.
* **Frontend :** **React (Vite + TypeScript)** avec éditeur de carte/nœuds (ex: **ReactFlow / @xyflow**), chat narratif en streaming, inspecteur d'état en temps réel.
* **Protocoles :** **Model Context Protocol (MCP)** pour l'outillage de Muse, **SSE/WebSockets** pour le streaming UI.

---

## 4. 🔄 La Boucle de Tour (*Turn Lifecycle*)

```text
[Joueur] (Texte court / intention brute)
   │
   ▼
[Backend Rust] ────> [Muse Glimmer (MJ)]
                        │
                        ├── 1. get_current_location() & get_present_npcs()
                        ├── 2. Décide de l'issue narrative et relationnelle
                        ├── 3. update_npc_state(id="elena", affinity=+10, emotion="troublée")
                        └── 4. Briefe Qwen : "Elena rougit et sourit, dialogue complice..."
                                │
                                ▼
                         [Qwen 3.8 (Plume)]
                         (Génère la prose immersive)
                                │
                                ▼
[Frontend React] <────── [Affichage texte en streaming + mise à jour jauges/carte]
```

---

## 5. 🚀 Prompt de Démarrage pour `/wayfinder`

Copie-colle le prompt ci-dessous dans la nouvelle session `agy cli` pour lancer la cartographie du projet :

```text
/wayfinder

Destination : Concevoir l'architecture complète et le plan de développement du projet JanusRP — un Sandbox narratif et gestionnaire de roleplay immersif autonome.

Stack technique :
- Backend : Rust (Axum, SQLx, Serde, Tokio, MCP Server)
- Database : PostgreSQL (Docker Compose)
- Frontend : React (Vite, TypeScript, ReactFlow)
- Agents : Meta Muse Glimmer 30B (MJ / Arbitre / Tools MCP) + Qwen 3.8 (Plume / Narration littéraire)

Objectif de la session :
Cartographier le graphe des tickets de décision (Schéma PostgreSQL, Spécification des tools MCP de Muse, Pipeline de streaming Rust <-> Muse <-> Qwen <-> React, et Éditeur de cartes/PNJ) pour dissiper le brouillard de guerre et préparer le découpage en vertical slices.
```
