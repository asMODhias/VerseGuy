use axum::body::to_bytes;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::sync::Arc;
use tempfile::TempDir;
use tower::util::ServiceExt; // for `oneshot` call
use verseguy_api::{build_app_with_services, store::TOKEN_STORE};
use verseguy_storage::Storage;

#[allow(clippy::disallowed_methods)]
#[tokio::test]
async fn ships_end_to_end() {
    // setup temp storage and fleet service
    let tmp = TempDir::new().unwrap();
    let storage = Storage::open(tmp.path()).unwrap();
    let fleet = plugins_base_fleet::service::FleetService::new(storage);
    let fleet = Arc::new(fleet);

    // token store used by build_app_with_services
    let store = TOKEN_STORE.clone();

    let app = build_app_with_services(store, fleet.clone());

    // create ship
    let payload = serde_json::json!({"owner_id":"o1","model":"Carrack","manufacturer":"Anvil"});
    let req = Request::builder()
        .method("POST")
        .uri("/ships")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // list ships
    let req = Request::builder()
        .method("GET")
        .uri("/ships/o1")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let list: Vec<plugins_base_fleet::types::Ship> = serde_json::from_slice(&body).unwrap();
    assert_eq!(list.len(), 1);

    // get ship
    let id = &list[0].id;
    let req = Request::builder()
        .method("GET")
        .uri(format!("/ships/o1/{}", id))
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // update ship (change name)
    let mut ship = list[0].clone();
    ship.name = Some("NewName".into());
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/ships/o1/{}", ship.id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&ship).unwrap()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // delete ship
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/ships/o1/{}", ship.id))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}
