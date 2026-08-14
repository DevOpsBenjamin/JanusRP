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

## 🚀 Démarrage Rapide

### 1. Prérequis
- Rust 1.80+ (`cargo`)
- Node.js 20+ (`npm`)
- Docker & Docker Compose

### 2. Démarrer la Base de Données
```bash
docker compose -f docker/docker-compose.yml up -d
```

### 3. Lancer le Backend Rust
```bash
cargo run --bin janus-server
```

### 4. Lancer le Frontend React
```bash
cd frontend
npm install
npm run dev
```

### 5. Exécuter les Tests
```bash
# Tests Backend
cargo test --workspace

# Build Frontend
cd frontend && npm run build
```
