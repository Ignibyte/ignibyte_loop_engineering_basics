//! Integration tests: drive the router directly with `oneshot`, no network.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use ignibyte_loop_engineering_basics::app;

#[tokio::test]
async fn healthz_returns_ok() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(std::str::from_utf8(&body).unwrap(), "ok");
}

#[tokio::test]
async fn notes_start_empty() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/api/notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let notes: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(notes, json!([]));
}

#[tokio::test]
async fn create_then_list_returns_the_note() {
    let application = app();

    let created = application
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"text":"first note"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::CREATED);

    let listed = application
        .oneshot(
            Request::builder()
                .uri("/api/notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = listed.into_body().collect().await.unwrap().to_bytes();
    let notes: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(notes[0]["id"], 1);
    assert_eq!(notes[0]["text"], "first note");
}
