#![allow(clippy::disallowed_methods)]
use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn create_and_add_ship_to_fleet() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    // Initially list should return empty array
    let req = must(
        Request::builder()
            .method("GET")
            .uri("/v1/fleets")
            .body(Body::empty()),
    );
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = must(body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&bytes));
    assert!(must_opt(v.get("fleets").and_then(|r| r.as_array()), "missing fleets").is_empty());

    // Create fleet
    let create_body = "{\"name\":\"TestFleet\"}".to_string();
    let req2 = must(
        Request::builder()
            .method("POST")
            .uri("/v1/fleets")
            .header("content-type", "application/json")
            .body(Body::from(create_body)),
    );
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);
    let bytes2 = must(body::to_bytes(resp2.into_body(), 1024 * 1024).await);
    let created: serde_json::Value = must(serde_json::from_slice(&bytes2));
    assert_eq!(
        must_opt(created.get("name").and_then(|n| n.as_str()), "missing name"),
        "TestFleet",
    );

    // Add ship to fleet
    let id = must_opt(created.get("id").and_then(|v| v.as_str()), "missing id");
    let uri_ship = format!("/v1/fleets/{}/ships", id);
    let add_ship_body = "{\"ship_type_id\":\"st-1\",\"name\":\"MyShip\"}".to_string();
    let req3 = must(
        Request::builder()
            .method("POST")
            .uri(uri_ship)
            .header("content-type", "application/json")
            .body(Body::from(add_ship_body)),
    );
    let resp3 = must(app.clone().oneshot(req3).await);
    assert_eq!(resp3.status(), StatusCode::OK);
    let bytes3 = must(body::to_bytes(resp3.into_body(), 1024 * 1024).await);
    let added: serde_json::Value = must(serde_json::from_slice(&bytes3));
    assert!(must_opt(
        added.get("ok").and_then(|v| v.as_bool()),
        "missing ok"
    ));

    // GET by id -> should contain one ship
    let uri = format!("/v1/fleets/{}", id);
    let req4 = must(
        Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()),
    );
    let resp4 = must(app.clone().oneshot(req4).await);
    assert_eq!(resp4.status(), StatusCode::OK);
    let bytes4 = must(body::to_bytes(resp4.into_body(), 1024 * 1024).await);
    let fetched: serde_json::Value = must(serde_json::from_slice(&bytes4));

    let ships = must_opt(
        fetched.get("ships").and_then(|v| v.as_array()),
        "missing ships",
    );
    assert_eq!(ships.len(), 1);
    let ship = &ships[0];
    assert_eq!(
        must_opt(
            ship.get("ship_type_id").and_then(|v| v.as_str()),
            "missing ship_type_id"
        ),
        "st-1"
    );
    assert_eq!(
        must_opt(ship.get("name").and_then(|v| v.as_str()), "missing name"),
        "MyShip"
    );
}
