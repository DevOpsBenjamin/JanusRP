use janus_db::{
    ARRIERE_COUR_LOCATION_ID, CHAMBRE_HAUTE_LOCATION_ID, EDGE_SALLE_ARRIERE_ID,
    EDGE_SALLE_CHAMBRE_ID, ELENA_NPC_ID, ELENA_REL_ID, GASTON_NPC_ID, GASTON_REL_ID,
    SALLE_COMMUNE_LOCATION_ID, VAL_CORBEAU_CAMPAIGN_ID,
};
use uuid::Uuid;

#[test]
fn test_starter_seed_constants_are_valid_and_distinct() {
    let ids = vec![
        VAL_CORBEAU_CAMPAIGN_ID,
        SALLE_COMMUNE_LOCATION_ID,
        ARRIERE_COUR_LOCATION_ID,
        CHAMBRE_HAUTE_LOCATION_ID,
        EDGE_SALLE_ARRIERE_ID,
        EDGE_SALLE_CHAMBRE_ID,
        ELENA_NPC_ID,
        GASTON_NPC_ID,
        ELENA_REL_ID,
        GASTON_REL_ID,
    ];

    // Verify all are non-nil
    for id in &ids {
        assert_ne!(*id, Uuid::nil());
    }

    // Verify all are distinct
    let mut unique_ids = ids.clone();
    unique_ids.sort();
    unique_ids.dedup();
    assert_eq!(unique_ids.len(), ids.len());
}

#[test]
fn test_migrations_sql_files_exist_and_are_valid() {
    let migration_0001 = include_str!("../migrations/0001_initial_schema.sql");
    let migration_0002 = include_str!("../migrations/0002_seed_val_corbeau.sql");

    assert!(migration_0001.contains("CREATE TABLE IF NOT EXISTS campaigns"));
    assert!(migration_0001.contains("CREATE TABLE IF NOT EXISTS locations"));
    assert!(migration_0001.contains("CREATE TABLE IF NOT EXISTS location_edges"));
    assert!(migration_0001.contains("CREATE TABLE IF NOT EXISTS npcs"));
    assert!(migration_0001.contains("CREATE TABLE IF NOT EXISTS npc_relationships"));
    assert!(migration_0001.contains("CREATE TABLE IF NOT EXISTS turns"));
    assert!(migration_0001.contains("CREATE TABLE IF NOT EXISTS narrative_events"));
    assert!(migration_0001.contains("idx_narrative_events_embedding"));

    assert!(migration_0002.contains("Les Brumes de Val-Corbeau"));
    assert!(migration_0002.contains("La Salle Commune"));
    assert!(migration_0002.contains("L''Arrière-Cour"));
    assert!(migration_0002.contains("La Chambre Haute"));
    assert!(migration_0002.contains("Elena la Tavernière"));
    assert!(migration_0002.contains("Gaston le Rôdeur"));
}
