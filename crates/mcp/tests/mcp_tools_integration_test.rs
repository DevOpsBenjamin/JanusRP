use janus_core::StateMutation;
use janus_db::{
    create_pool, run_migrations, seed_val_corbeau, ARRIERE_COUR_LOCATION_ID,
    CHAMBRE_HAUTE_LOCATION_ID, ELENA_NPC_ID, GASTON_NPC_ID, SALLE_COMMUNE_LOCATION_ID,
    VAL_CORBEAU_CAMPAIGN_ID,
};
use janus_mcp::{McpError, McpServer, ToolCall};
use serde_json::json;
use uuid::Uuid;

async fn setup_test_db() -> Option<janus_db::PgPool> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/janusrp".to_string());

    match create_pool(&db_url).await {
        Ok(pool) => {
            if run_migrations(&pool).await.is_ok() && seed_val_corbeau(&pool).await.is_ok() {
                Some(pool)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[tokio::test]
async fn test_mcp_get_location_context_and_secrets() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => {
            eprintln!("PostgreSQL not reachable, skipping live DB test");
            return;
        }
    };

    let server = McpServer::new(pool);

    // 1. Get location context without explicit location_id (default to campaign current location)
    let call = ToolCall {
        name: "get_location_context".to_string(),
        arguments: json!({
            "include_secrets": false
        }),
    };

    let res = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &call)
        .await
        .expect("get_location_context should succeed");

    assert!(res.success);
    assert_eq!(res.tool_name, "get_location_context");
    assert!(res.mutation.is_none());

    let payload = res.result;
    assert_eq!(
        payload["location_id"].as_str().unwrap(),
        SALLE_COMMUNE_LOCATION_ID.to_string()
    );
    assert_eq!(payload["slug"].as_str().unwrap(), "salle-commune");
    assert!(payload["secrets"].is_null());

    let npcs = payload["npcs_present"].as_array().unwrap();
    assert!(!npcs.is_empty());
    assert!(npcs
        .iter()
        .any(|n| n["npc_id"].as_str().unwrap() == ELENA_NPC_ID.to_string()));

    let connected = payload["connected_locations"].as_array().unwrap();
    assert!(!connected.is_empty());

    // 2. Get location context with secrets included
    let call_secrets = ToolCall {
        name: "get_location_context".to_string(),
        arguments: json!({
            "location_id": SALLE_COMMUNE_LOCATION_ID.to_string(),
            "include_secrets": true
        }),
    };

    let res_secrets = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &call_secrets)
        .await
        .expect("get_location_context with secrets should succeed");

    assert!(res_secrets.result["secrets"].is_string());
    assert!(res_secrets.result["secrets"]
        .as_str()
        .unwrap()
        .contains("trappe secrète"));
}

#[tokio::test]
async fn test_mcp_inspect_npc_details() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => return,
    };

    let server = McpServer::new(pool);

    let call = ToolCall {
        name: "inspect_npc_details".to_string(),
        arguments: json!({
            "npc_id": ELENA_NPC_ID.to_string(),
            "query_focus": "secret_agenda"
        }),
    };

    let res = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &call)
        .await
        .expect("inspect_npc_details should succeed");

    assert!(res.success);
    let data = res.result;
    assert_eq!(data["npc_id"].as_str().unwrap(), ELENA_NPC_ID.to_string());
    assert_eq!(data["slug"].as_str().unwrap(), "elena");
    assert!(data["secret_agenda"].as_str().unwrap().contains("fugitifs"));
    assert_eq!(data["query_focus"].as_str().unwrap(), "secret_agenda");
    assert!(data["relationship"]["mood"].is_string());
}

#[tokio::test]
async fn test_mcp_update_npc_relation() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => return,
    };

    let server = McpServer::new(pool);

    let call = ToolCall {
        name: "update_npc_relation".to_string(),
        arguments: json!({
            "npc_id": ELENA_NPC_ID.to_string(),
            "delta_affinity": 25,
            "delta_trust": 10,
            "mood": "enthousiaste",
            "reason": "Le joueur a offert son aide spontanément."
        }),
    };

    let res = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 2, &call)
        .await
        .expect("update_npc_relation should succeed");

    assert!(res.success);
    assert_eq!(res.tool_name, "update_npc_relation");

    // Mutation verification
    if let Some(StateMutation::RelationshipUpdate {
        npc_id,
        affinity,
        trust,
        mood,
        delta_affinity,
        delta_trust,
        reason,
        ..
    }) = res.mutation
    {
        assert_eq!(npc_id, ELENA_NPC_ID);
        assert_eq!(affinity, 25);
        assert_eq!(trust, 30); // 20 initial + 10
        assert_eq!(mood, "enthousiaste");
        assert_eq!(delta_affinity, Some(25));
        assert_eq!(delta_trust, Some(10));
        assert_eq!(reason, "Le joueur a offert son aide spontanément.");
    } else {
        panic!("Expected StateMutation::RelationshipUpdate");
    }
}

#[tokio::test]
async fn test_mcp_move_to_location_topological_and_force() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => return,
    };

    let server = McpServer::new(pool.clone());

    // Reset player position to Salle Commune
    janus_db::campaigns::update_current_location(
        &pool,
        VAL_CORBEAU_CAMPAIGN_ID,
        Some(SALLE_COMMUNE_LOCATION_ID),
    )
    .await
    .unwrap();

    // 1. Trying to move to Chambre Haute (locked edge) without force should fail
    let locked_call = ToolCall {
        name: "move_to_location".to_string(),
        arguments: json!({
            "target_location_id": CHAMBRE_HAUTE_LOCATION_ID.to_string(),
            "force": false
        }),
    };

    let err = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &locked_call)
        .await
        .unwrap_err();

    match err {
        McpError::Execution(msg) => {
            assert!(msg.contains("verrouillé"));
        }
        other => panic!("Expected McpError::Execution, got {:?}", other),
    }

    // 2. Moving to Arrière-Cour (unlocked adjacent edge) should succeed
    let valid_move = ToolCall {
        name: "move_to_location".to_string(),
        arguments: json!({
            "target_location_id": ARRIERE_COUR_LOCATION_ID.to_string(),
            "movement_narration_hint": "Passe par la porte de service vers la cour"
        }),
    };

    let move_res = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &valid_move)
        .await
        .expect("Movement to adjacent Arrière-Cour should succeed");

    assert!(move_res.success);
    if let Some(StateMutation::LocationChange {
        location_id,
        location_name,
        narration_hint,
    }) = move_res.mutation
    {
        assert_eq!(location_id, ARRIERE_COUR_LOCATION_ID);
        assert_eq!(location_name, "L'Arrière-Cour");
        assert_eq!(
            narration_hint,
            Some("Passe par la porte de service vers la cour".to_string())
        );
    } else {
        panic!("Expected StateMutation::LocationChange");
    }

    // Verify campaign location updated in DB
    let campaign = janus_db::campaigns::get_by_id(&pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        campaign.current_location_id,
        Some(ARRIERE_COUR_LOCATION_ID)
    );

    // 3. Forced move to Chambre Haute should succeed despite lock/distance
    let forced_move = ToolCall {
        name: "move_to_location".to_string(),
        arguments: json!({
            "target_location_id": CHAMBRE_HAUTE_LOCATION_ID.to_string(),
            "force": true,
            "movement_narration_hint": "Téléportation magique dans la chambre"
        }),
    };

    let forced_res = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 2, &forced_move)
        .await
        .expect("Forced movement should bypass topology");

    assert!(forced_res.success);
    let campaign_after = janus_db::campaigns::get_by_id(&pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        campaign_after.current_location_id,
        Some(CHAMBRE_HAUTE_LOCATION_ID)
    );
}

#[tokio::test]
async fn test_mcp_log_event() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => return,
    };

    let server = McpServer::new(pool.clone());

    let call = ToolCall {
        name: "log_event".to_string(),
        arguments: json!({
            "summary": "Découverte de documents suspects cachés sous une latte de plancher.",
            "significance": "major",
            "involved_npc_ids": [GASTON_NPC_ID.to_string()],
            "tags": ["secret", "intrigue", "lettres"]
        }),
    };

    let res = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 3, &call)
        .await
        .expect("log_event should succeed");

    assert!(res.success);
    assert_eq!(res.tool_name, "log_event");

    if let Some(StateMutation::EventLogged {
        event_id,
        summary,
        significance,
    }) = res.mutation
    {
        assert_ne!(event_id, Uuid::nil());
        assert_eq!(
            summary,
            "Découverte de documents suspects cachés sous une latte de plancher."
        );
        assert_eq!(significance, "major");

        // Verify persisted in DB
        let db_event = janus_db::events::get_by_id(&pool, event_id)
            .await
            .unwrap()
            .expect("Event should exist in database");
        assert_eq!(db_event.campaign_id, VAL_CORBEAU_CAMPAIGN_ID);
        assert_eq!(db_event.tags, vec!["secret", "intrigue", "lettres"]);
    } else {
        panic!("Expected StateMutation::EventLogged");
    }
}

#[tokio::test]
async fn test_mcp_execute_tools_batch() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => return,
    };

    let server = McpServer::new(pool);

    let calls = vec![
        ToolCall {
            name: "update_npc_relation".to_string(),
            arguments: json!({
                "npc_id": GASTON_NPC_ID.to_string(),
                "delta_affinity": -5,
                "delta_trust": -10,
                "mood": "très méfiant",
                "reason": "Regard insistant vers sa ceinture"
            }),
        },
        ToolCall {
            name: "log_event".to_string(),
            arguments: json!({
                "summary": "Tension palpable avec Gaston dans l'auberge.",
                "significance": "minor"
            }),
        },
    ];

    let results = server
        .execute_tools(VAL_CORBEAU_CAMPAIGN_ID, 4, &calls)
        .await
        .expect("Batch execution should succeed");

    assert_eq!(results.len(), 2);
    assert!(results[0].mutation.is_some());
    assert!(results[1].mutation.is_some());
}

#[tokio::test]
async fn test_mcp_error_handling() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => return,
    };

    let server = McpServer::new(pool);

    // 1. Unknown tool
    let unknown_call = ToolCall {
        name: "cast_fireball".to_string(),
        arguments: json!({}),
    };
    let err = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &unknown_call)
        .await
        .unwrap_err();
    assert!(matches!(err, McpError::ToolNotFound(_)));

    // 2. Invalid arguments (missing required npc_id)
    let invalid_call = ToolCall {
        name: "inspect_npc_details".to_string(),
        arguments: json!({}),
    };
    let err = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &invalid_call)
        .await
        .unwrap_err();
    assert!(matches!(err, McpError::InvalidArguments { .. }));

    // 3. NPC not found
    let non_existent_npc = ToolCall {
        name: "inspect_npc_details".to_string(),
        arguments: json!({
            "npc_id": Uuid::new_v4().to_string()
        }),
    };
    let err = server
        .execute_tool(VAL_CORBEAU_CAMPAIGN_ID, 1, &non_existent_npc)
        .await
        .unwrap_err();
    assert!(matches!(err, McpError::Execution(_)));
}
