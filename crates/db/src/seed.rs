use janus_core::Campaign;
use sqlx::PgPool;
use tracing::info;
use uuid::{uuid, Uuid};

use crate::error::DbError;

pub const VAL_CORBEAU_CAMPAIGN_ID: Uuid = uuid!("a0000000-0000-0000-0000-000000000001");
pub const SALLE_COMMUNE_LOCATION_ID: Uuid = uuid!("b0000000-0000-0000-0000-000000000001");
pub const ARRIERE_COUR_LOCATION_ID: Uuid = uuid!("b0000000-0000-0000-0000-000000000002");
pub const CHAMBRE_HAUTE_LOCATION_ID: Uuid = uuid!("b0000000-0000-0000-0000-000000000003");
pub const EDGE_SALLE_ARRIERE_ID: Uuid = uuid!("c0000000-0000-0000-0000-000000000001");
pub const EDGE_SALLE_CHAMBRE_ID: Uuid = uuid!("c0000000-0000-0000-0000-000000000002");
pub const ELENA_NPC_ID: Uuid = uuid!("d0000000-0000-0000-0000-000000000001");
pub const GASTON_NPC_ID: Uuid = uuid!("d0000000-0000-0000-0000-000000000002");
pub const ELENA_REL_ID: Uuid = uuid!("e0000000-0000-0000-0000-000000000001");
pub const GASTON_REL_ID: Uuid = uuid!("e0000000-0000-0000-0000-000000000002");

/// Seeding programmatique et idempotent de la campagne de référence "Les Brumes de Val-Corbeau"
pub async fn seed_val_corbeau(pool: &PgPool) -> Result<Campaign, DbError> {
    info!("Seeding starter campaign 'Les Brumes de Val-Corbeau'...");

    // 1. Campagne
    sqlx::query(
        r#"
        INSERT INTO campaigns (
            id, title, description, system_prompt_theme, player_name, turn_count, metadata
        ) VALUES (
            $1,
            'Les Brumes de Val-Corbeau',
            'Une auberge isolée au cœur des forêts brumeuses du Val-Corbeau, carrefour de voyageurs, de marchands et de fugitifs.',
            'Ambiance dark fantasy feutrée, mystère et intrigues locales.',
            'Aventurier',
            0,
            '{"theme": "dark_fantasy", "region": "Val-Corbeau"}'::jsonb
        ) ON CONFLICT (id) DO UPDATE SET
            title = EXCLUDED.title,
            description = EXCLUDED.description,
            system_prompt_theme = EXCLUDED.system_prompt_theme;
        "#,
    )
    .bind(VAL_CORBEAU_CAMPAIGN_ID)
    .execute(pool)
    .await?;

    // 2. Lieux
    sqlx::query(
        r#"
        INSERT INTO locations (
            id, campaign_id, slug, name, description, atmosphere, secrets, position_x, position_y, props, metadata
        ) VALUES 
        (
            $1, $4, 'salle-commune', 'La Salle Commune',
            'L''atmosphère est tiède et saturée par l''odeur de suif, de bière aigre et de tourbe brûlée. Un grand feu crépite dans l''âtre en pierre, projetant des ombres dansantes sur les tables de chêne massif.',
            'Chaleureux mais méfiant', 'Une trappe secrète mène à la cave sous le comptoir.',
            0.0, 0.0, '["cheminée", "comptoir en chêne", "chopes de bois"]'::jsonb, '{"is_starting_location": true}'::jsonb
        ),
        (
            $2, $4, 'arriere-cour', 'L''Arrière-Cour',
            'Une ruelle boueuse et sombre battue par une pluie fine et glaciale. Des caisses empilées et des tonneaux éventrés offrent des cachettes idéales.',
            'Sombre et glacial', 'Des traces de pas récentes mènent vers les marais.',
            250.0, 0.0, '["tonneaux éventrés", "palissade de bois"]'::jsonb, '{}'::jsonb
        ),
        (
            $3, $4, 'chambre-haute', 'La Chambre Haute',
            'Une chambre sous les combles aux poutres noircies par le temps. Le plancher grince à chaque pas et le silence n''est troublé que par le vent qui siffle à travers les volets.',
            'Silencieux et poussiéreux', 'Des documents scellés de la garde royale sont cachés sous le plancher.',
            0.0, -200.0, '["lit à baldaquin", "coffre en fer"]'::jsonb, '{}'::jsonb
        )
        ON CONFLICT (campaign_id, slug) DO UPDATE SET
            name = EXCLUDED.name,
            description = EXCLUDED.description,
            atmosphere = EXCLUDED.atmosphere,
            secrets = EXCLUDED.secrets,
            position_x = EXCLUDED.position_x,
            position_y = EXCLUDED.position_y,
            props = EXCLUDED.props;
        "#,
    )
    .bind(SALLE_COMMUNE_LOCATION_ID)
    .bind(ARRIERE_COUR_LOCATION_ID)
    .bind(CHAMBRE_HAUTE_LOCATION_ID)
    .bind(VAL_CORBEAU_CAMPAIGN_ID)
    .execute(pool)
    .await?;

    // 3. Mettre à jour current_location_id
    sqlx::query(
        r#"
        UPDATE campaigns 
        SET current_location_id = $1 
        WHERE id = $2 AND (current_location_id IS NULL OR current_location_id != $1);
        "#,
    )
    .bind(SALLE_COMMUNE_LOCATION_ID)
    .bind(VAL_CORBEAU_CAMPAIGN_ID)
    .execute(pool)
    .await?;

    // 4. Arêtes
    sqlx::query(
        r#"
        INSERT INTO location_edges (
            id, campaign_id, source_location_id, target_location_id,
            bidirectional, travel_description, is_locked, lock_reason, metadata
        ) VALUES 
        (
            $1, $5, $2, $3, true,
            'Une lourde porte de service relie la salle commune à l''arrière-cour boueuse.',
            false, NULL, '{}'::jsonb
        ),
        (
            $4, $5, $2, $6, true,
            'Un escalier en colimaçon étroit monte vers la chambre sous les toits.',
            true, 'La porte en chêne massif est fermée à double tour par une serrure ouvragée.', '{}'::jsonb
        )
        ON CONFLICT (source_location_id, target_location_id) DO UPDATE SET
            travel_description = EXCLUDED.travel_description,
            is_locked = EXCLUDED.is_locked,
            lock_reason = EXCLUDED.lock_reason;
        "#,
    )
    .bind(EDGE_SALLE_ARRIERE_ID)
    .bind(SALLE_COMMUNE_LOCATION_ID)
    .bind(ARRIERE_COUR_LOCATION_ID)
    .bind(EDGE_SALLE_CHAMBRE_ID)
    .bind(VAL_CORBEAU_CAMPAIGN_ID)
    .bind(CHAMBRE_HAUTE_LOCATION_ID)
    .execute(pool)
    .await?;

    // 5. PNJ
    sqlx::query(
        r#"
        INSERT INTO npcs (
            id, campaign_id, current_location_id, slug, name, title,
            personality_traits, secret_agenda, background, is_alive, is_active, metadata
        ) VALUES 
        (
            $1, $3, $4, 'elena', 'Elena la Tavernière', 'Tavernière de Val-Corbeau',
            '["accueillante", "prudente", "observatrice"]'::jsonb,
            'Recèle des vivres et cache des lettres pour les fugitifs de la rébellion.',
            'Tient l''auberge depuis 15 ans après la disparition mystérieuse de son époux.',
            true, true, '{}'::jsonb
        ),
        (
            $2, $3, $5, 'gaston', 'Gaston le Rôdeur', 'Pisteur & Contrebandier',
            '["méfiant", "laconique", "coriace"]'::jsonb,
            'Surveille les allées et venues pour une guilde rivale et cherche un contact évadé.',
            'Ancien éclaireur de l''armée royale, survit de petits trafics.',
            true, true, '{}'::jsonb
        )
        ON CONFLICT (campaign_id, slug) DO UPDATE SET
            name = EXCLUDED.name,
            title = EXCLUDED.title,
            current_location_id = EXCLUDED.current_location_id,
            personality_traits = EXCLUDED.personality_traits,
            secret_agenda = EXCLUDED.secret_agenda,
            background = EXCLUDED.background;
        "#,
    )
    .bind(ELENA_NPC_ID)
    .bind(GASTON_NPC_ID)
    .bind(VAL_CORBEAU_CAMPAIGN_ID)
    .bind(SALLE_COMMUNE_LOCATION_ID)
    .bind(ARRIERE_COUR_LOCATION_ID)
    .execute(pool)
    .await?;

    // 6. Relations PNJ
    sqlx::query(
        r#"
        INSERT INTO npc_relationships (
            id, npc_id, affinity, trust, mood, last_interaction_turn, interaction_summary, metadata
        ) VALUES 
        (
            $1, $3, 0, 20, 'bienveillante mais vigilante', 0,
            'Elena accueille le nouveau venu avec attention mais garde ses réserves.', '{}'::jsonb
        ),
        (
            $2, $4, -20, 0, 'méfiant', 0,
            'Gaston observe l''inconnu avec suspicion depuis l''ombre de la cour.', '{}'::jsonb
        )
        ON CONFLICT (npc_id) DO UPDATE SET
            affinity = EXCLUDED.affinity,
            trust = EXCLUDED.trust,
            mood = EXCLUDED.mood;
        "#,
    )
    .bind(ELENA_REL_ID)
    .bind(GASTON_REL_ID)
    .bind(ELENA_NPC_ID)
    .bind(GASTON_NPC_ID)
    .execute(pool)
    .await?;

    let campaign = crate::campaigns::get_by_id(pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await?
        .ok_or_else(|| DbError::NotFound {
            table: "campaigns",
            id: VAL_CORBEAU_CAMPAIGN_ID.to_string(),
        })?;

    info!("Starter campaign seeded successfully.");
    Ok(campaign)
}
