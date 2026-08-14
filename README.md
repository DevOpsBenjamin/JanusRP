# JanusRP — Sandbox Narratif & Roleplay Immersif

JanusRP est une plateforme de jeu de rôle immersif et sandbox narratif alimentée par deux modèles d'IA spécialisés :
- **Meta Muse Glimmer 30B (Le Maître du Jeu / Arbitre)** : Arbitrage logique, consultation et mutation d'état via Model Context Protocol (MCP in-process).
- **Qwen 3.8 (La Plume / Le Narrateur)** : Rédaction littéraire sensorielle en streaming SSE avec DSL XML balisé (`<dialogue>`, `<thought>`, `<sensory>`, `<document>`).

## 📁 Structure du Monorepo

```text
/
├── crates/
│   ├── core/         # Types de domaine (Campaign, Location, NPC, Turn), événements SSE, erreurs
│   ├── db/           # Schéma PostgreSQL 16 + pgvector, migrations SQLx, pool de connexions
│   ├── mcp/          # Serveur MCP in-process et catalogue des 5 outils de tour
│   ├── llm/          # Trait LlmClient, MockLlmClient et HttpLlmClient (vLLM / Ollama / Aphrodite)
│   └── server/       # Serveur Axum, endpoints REST / SSE, orchestration Tokio
├── frontend/         # Console React 18, Vite, TypeScript, ReactFlow, Zustand, TailwindCSS
├── docker/           # PostgreSQL 16 + pgvector (docker-compose.yml)
├── docs/             # ADRs d'architecture et guides agents
└── CONTEXT.md        # Glossaire du langage ubiquitaire du domaine
```

## 🚀 Démarrage

### 1. 🐳 Mode Full Docker (Recommandé — Zéro installation locale requise)

#### A. Mode Utilisateur / Standalone (Plug & Play)
Lance toute la stack (PostgreSQL + Backend Rust + Frontend Web Nginx) :
```bash
docker compose -f docker/docker-compose.yml up -d
```
- Interface Web : [http://localhost:5173](http://localhost:5173)
- API Backend : [http://localhost:3000/health](http://localhost:3000/health)

#### B. Mode Développeur (Live-Reload à chaud)
Lance toute la stack dans Docker avec rechargement automatique du code (`cargo watch` et Vite HMR) :
```bash
docker compose -f docker/docker-compose.dev.yml up
```

---

### 2. 💻 Mode Développement Local (Sans conteneurisation du code)

1. Démarrer uniquement la base de données :
   ```bash
   docker compose -f docker/docker-compose.yml up -d postgres
   ```
2. Lancer le backend Rust :
   ```bash
   cargo run --bin janus-server
   ```
3. Lancer le frontend React :
   ```bash
   cd frontend && npm install && npm run dev
   ```

---

### 🧪 Exécuter les Tests
```bash
# Tests Backend Rust
cargo test --workspace

# Build Frontend
cd frontend && npm run build
```
