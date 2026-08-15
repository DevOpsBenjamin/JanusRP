pub mod campaigns;
pub mod error;
pub mod events;
pub mod location_edges;
pub mod locations;
pub mod migrations;
pub mod npcs;
pub mod pool;
pub mod seed;
pub mod turns;

pub use error::DbError;
pub use migrations::run_migrations;
pub use pool::create_pool;
pub use seed::{
    seed_val_corbeau, ARRIERE_COUR_LOCATION_ID, CHAMBRE_HAUTE_LOCATION_ID, EDGE_SALLE_ARRIERE_ID,
    EDGE_SALLE_CHAMBRE_ID, ELENA_NPC_ID, ELENA_REL_ID, GASTON_NPC_ID, GASTON_REL_ID,
    SALLE_COMMUNE_LOCATION_ID, VAL_CORBEAU_CAMPAIGN_ID,
};
pub use sqlx::PgPool;
