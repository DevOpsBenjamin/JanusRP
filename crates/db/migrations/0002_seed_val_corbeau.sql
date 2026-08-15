-- Migration 0002: Starter Campaign Seed ("Les Brumes de Val-Corbeau")

-- 1. Insérer la Campagne
INSERT INTO campaigns (
    id,
    title,
    description,
    system_prompt_theme,
    player_name,
    turn_count,
    metadata
) VALUES (
    'a0000000-0000-0000-0000-000000000001',
    'Les Brumes de Val-Corbeau',
    'Une auberge isolée au cœur des forêts brumeuses du Val-Corbeau, carrefour de voyageurs, de marchands et de fugitifs.',
    'Ambiance dark fantasy feutrée, mystère et intrigues locales.',
    'Aventurier',
    0,
    '{"theme": "dark_fantasy", "region": "Val-Corbeau"}'::jsonb
) ON CONFLICT (id) DO NOTHING;

-- 2. Insérer les Lieux
INSERT INTO locations (
    id,
    campaign_id,
    slug,
    name,
    description,
    atmosphere,
    secrets,
    position_x,
    position_y,
    props,
    metadata
) VALUES 
(
    'b0000000-0000-0000-0000-000000000001',
    'a0000000-0000-0000-0000-000000000001',
    'salle-commune',
    'La Salle Commune',
    'L''atmosphère est tiède et saturée par l''odeur de suif, de bière aigre et de tourbe brûlée. Un grand feu crépite dans l''âtre en pierre, projetant des ombres dansantes sur les tables de chêne massif.',
    'Chaleureux mais méfiant',
    'Une trappe secrète mène à la cave sous le comptoir.',
    0.0,
    0.0,
    '["cheminée", "comptoir en chêne", "chopes de bois"]'::jsonb,
    '{"is_starting_location": true}'::jsonb
),
(
    'b0000000-0000-0000-0000-000000000002',
    'a0000000-0000-0000-0000-000000000001',
    'arriere-cour',
    'L''Arrière-Cour',
    'Une ruelle boueuse et sombre battue par une pluie fine et glaciale. Des caisses empilées et des tonneaux éventrés offrent des cachettes idéales.',
    'Sombre et glacial',
    'Des traces de pas récentes mènent vers les marais.',
    250.0,
    0.0,
    '["tonneaux éventrés", "palissade de bois"]'::jsonb,
    '{}'::jsonb
),
(
    'b0000000-0000-0000-0000-000000000003',
    'a0000000-0000-0000-0000-000000000001',
    'chambre-haute',
    'La Chambre Haute',
    'Une chambre sous les combles aux poutres noircies par le temps. Le plancher grince à chaque pas et le silence n''est troublé que par le vent qui siffle à travers les volets.',
    'Silencieux et poussiéreux',
    'Des documents scellés de la garde royale sont cachés sous le plancher.',
    0.0,
    -200.0,
    '["lit à baldaquin", "coffre en fer"]'::jsonb,
    '{}'::jsonb
) ON CONFLICT (campaign_id, slug) DO NOTHING;

-- 3. Mettre à jour le lieu de départ de la campagne
UPDATE campaigns 
SET current_location_id = 'b0000000-0000-0000-0000-000000000001'
WHERE id = 'a0000000-0000-0000-0000-000000000001' AND current_location_id IS NULL;

-- 4. Insérer les Arêtes de Navigation Spatiale
INSERT INTO location_edges (
    id,
    campaign_id,
    source_location_id,
    target_location_id,
    bidirectional,
    travel_description,
    is_locked,
    lock_reason,
    metadata
) VALUES 
(
    'c0000000-0000-0000-0000-000000000001',
    'a0000000-0000-0000-0000-000000000001',
    'b0000000-0000-0000-0000-000000000001',
    'b0000000-0000-0000-0000-000000000002',
    true,
    'Une lourde porte de service relie la salle commune à l''arrière-cour boueuse.',
    false,
    NULL,
    '{}'::jsonb
),
(
    'c0000000-0000-0000-0000-000000000002',
    'a0000000-0000-0000-0000-000000000001',
    'b0000000-0000-0000-0000-000000000001',
    'b0000000-0000-0000-0000-000000000003',
    true,
    'Un escalier en colimaçon étroit monte vers la chambre sous les toits.',
    true,
    'La porte en chêne massif est fermée à double tour par une serrure ouvragée.',
    '{}'::jsonb
) ON CONFLICT (source_location_id, target_location_id) DO NOTHING;

-- 5. Insérer les PNJ
INSERT INTO npcs (
    id,
    campaign_id,
    current_location_id,
    slug,
    name,
    title,
    personality_traits,
    secret_agenda,
    background,
    is_alive,
    is_active,
    metadata
) VALUES 
(
    'd0000000-0000-0000-0000-000000000001',
    'a0000000-0000-0000-0000-000000000001',
    'b0000000-0000-0000-0000-000000000001',
    'elena',
    'Elena la Tavernière',
    'Tavernière de Val-Corbeau',
    '["accueillante", "prudente", "observatrice"]'::jsonb,
    'Recèle des vivres et cache des lettres pour les fugitifs de la rébellion.',
    'Tient l''auberge depuis 15 ans après la disparition mystérieuse de son époux.',
    true,
    true,
    '{}'::jsonb
),
(
    'd0000000-0000-0000-0000-000000000002',
    'a0000000-0000-0000-0000-000000000001',
    'b0000000-0000-0000-0000-000000000002',
    'gaston',
    'Gaston le Rôdeur',
    'Pisteur & Contrebandier',
    '["méfiant", "laconique", "coriace"]'::jsonb,
    'Surveille les allées et venues pour une guilde rivale et cherche un contact évadé.',
    'Ancien éclaireur de l''armée royale, survit de petits trafics.',
    true,
    true,
    '{}'::jsonb
) ON CONFLICT (campaign_id, slug) DO NOTHING;

-- 6. Insérer les Relations et Jauges PNJ
INSERT INTO npc_relationships (
    id,
    npc_id,
    affinity,
    trust,
    mood,
    last_interaction_turn,
    interaction_summary,
    metadata
) VALUES 
(
    'e0000000-0000-0000-0000-000000000001',
    'd0000000-0000-0000-0000-000000000001',
    0,
    20,
    'bienveillante mais vigilante',
    0,
    'Elena accueille le nouveau venu avec attention mais garde ses réserves.',
    '{}'::jsonb
),
(
    'e0000000-0000-0000-0000-000000000002',
    'd0000000-0000-0000-0000-000000000002',
    -20,
    0,
    'méfiant',
    0,
    'Gaston observe l''inconnu avec suspicion depuis l''ombre de la cour.',
    '{}'::jsonb
) ON CONFLICT (npc_id) DO NOTHING;
