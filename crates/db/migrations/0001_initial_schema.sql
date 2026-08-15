-- Migration 0001: Initial Schema (JanusRP)
-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";

-- 1. Table des Campagnes / Mondes
CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    system_prompt_theme TEXT,
    player_name VARCHAR(128) NOT NULL DEFAULT 'Aventurier',
    current_location_id UUID,
    turn_count INTEGER NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Lieux (Nœuds du Graphe)
CREATE TABLE IF NOT EXISTS locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    slug VARCHAR(64) NOT NULL,
    name VARCHAR(128) NOT NULL,
    description TEXT NOT NULL,
    atmosphere TEXT,
    secrets TEXT,
    position_x REAL NOT NULL DEFAULT 0.0,
    position_y REAL NOT NULL DEFAULT 0.0,
    props JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_location_campaign_slug UNIQUE (campaign_id, slug)
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'fk_campaign_current_location'
    ) THEN
        ALTER TABLE campaigns 
            ADD CONSTRAINT fk_campaign_current_location 
            FOREIGN KEY (current_location_id) REFERENCES locations(id) ON DELETE SET NULL;
    END IF;
END $$;

-- 3. Connexions entre Lieux (Arêtes du Graphe)
CREATE TABLE IF NOT EXISTS location_edges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    source_location_id UUID NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    target_location_id UUID NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    bidirectional BOOLEAN NOT NULL DEFAULT TRUE,
    travel_description TEXT,
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    lock_reason TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_location_edge UNIQUE (source_location_id, target_location_id)
);

-- 4. PNJ (Personnages Non-Joueurs)
CREATE TABLE IF NOT EXISTS npcs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    current_location_id UUID REFERENCES locations(id) ON DELETE SET NULL,
    slug VARCHAR(64) NOT NULL,
    name VARCHAR(128) NOT NULL,
    title VARCHAR(128),
    personality_traits JSONB NOT NULL DEFAULT '[]'::jsonb,
    secret_agenda TEXT,
    background TEXT,
    is_alive BOOLEAN NOT NULL DEFAULT TRUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_npc_campaign_slug UNIQUE (campaign_id, slug)
);

-- 5. Jauges et Relations PNJ-PJ
CREATE TABLE IF NOT EXISTS npc_relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    npc_id UUID NOT NULL REFERENCES npcs(id) ON DELETE CASCADE UNIQUE,
    affinity INTEGER NOT NULL DEFAULT 0 CHECK (affinity >= -100 AND affinity <= 100),
    trust INTEGER NOT NULL DEFAULT 0 CHECK (trust >= -100 AND trust <= 100),
    mood VARCHAR(64) NOT NULL DEFAULT 'neutre',
    last_interaction_turn INTEGER NOT NULL DEFAULT 0,
    interaction_summary TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 6. Historique Séquentiel des Tours
CREATE TABLE IF NOT EXISTS turns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    turn_number INTEGER NOT NULL,
    player_input TEXT NOT NULL,
    mj_reasoning TEXT,
    mj_briefing TEXT,
    final_narration TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_turn_campaign_number UNIQUE (campaign_id, turn_number)
);

-- 7. Journal d'Événements Persistants & Mémoire Vectorielle
CREATE TABLE IF NOT EXISTS narrative_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    turn_id UUID REFERENCES turns(id) ON DELETE SET NULL,
    location_id UUID REFERENCES locations(id) ON DELETE SET NULL,
    summary TEXT NOT NULL,
    significance VARCHAR(32) NOT NULL DEFAULT 'notable' CHECK (significance IN ('minor', 'notable', 'major', 'critical')),
    involved_npc_ids UUID[] NOT NULL DEFAULT '{}',
    tags TEXT[] NOT NULL DEFAULT '{}',
    embedding vector(1536),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index pour la recherche vectorielle (HNSW avec distance cosinus)
CREATE INDEX IF NOT EXISTS idx_narrative_events_embedding 
    ON narrative_events 
    USING hnsw (embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);

-- Index relationnels clés
CREATE INDEX IF NOT EXISTS idx_locations_campaign ON locations(campaign_id);
CREATE INDEX IF NOT EXISTS idx_location_edges_source ON location_edges(source_location_id);
CREATE INDEX IF NOT EXISTS idx_location_edges_target ON location_edges(target_location_id);
CREATE INDEX IF NOT EXISTS idx_npcs_location ON npcs(current_location_id) WHERE is_alive = TRUE;
CREATE INDEX IF NOT EXISTS idx_turns_campaign_turn ON turns(campaign_id, turn_number DESC);
CREATE INDEX IF NOT EXISTS idx_events_campaign ON narrative_events(campaign_id);
