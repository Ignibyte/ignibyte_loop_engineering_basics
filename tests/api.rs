//! Integration tests: drive the router directly with `oneshot`, no network.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tempfile::tempdir;
use tower::ServiceExt;

use ignibyte_loop_engineering_basics::{app, app_with_state, state::AppState};

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

// --- Persistence (the note-persistence spec, one test per EARS criterion) ---

#[tokio::test]
async fn loads_existing_notes_on_startup() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("notes.json");
    std::fs::write(&path, r#"[{"id":1,"text":"hello from disk"}]"#).unwrap();

    let response = app_with_state(AppState::new(Some(path)))
        .oneshot(
            Request::builder()
                .uri("/api/notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let notes: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(notes[0]["text"], "hello from disk");
}

#[tokio::test]
async fn create_persists_to_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("notes.json");

    let response = app_with_state(AppState::new(Some(path.clone())))
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"text":"persist me"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let on_disk = std::fs::read_to_string(&path).unwrap();
    assert!(on_disk.contains("persist me"));
}

#[tokio::test]
async fn missing_file_starts_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("does-not-exist.json");

    let response = app_with_state(AppState::new(Some(path)))
        .oneshot(
            Request::builder()
                .uri("/api/notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let notes: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(notes, json!([]));
}

#[tokio::test]
async fn malformed_file_starts_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("notes.json");
    std::fs::write(&path, "this is not json").unwrap();

    let response = app_with_state(AppState::new(Some(path)))
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
async fn write_failure_returns_500_and_keeps_memory() {
    // Point the notes "file" at a directory, so every write fails.
    let dir = tempdir().unwrap();
    let application = app_with_state(AppState::new(Some(dir.path().to_path_buf())));

    let created = application
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"text":"nope"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::INTERNAL_SERVER_ERROR);

    // The failed write rolled back, so the list is still empty.
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
    assert_eq!(notes, json!([]));
}

// --- The notes UI (the notes-ui spec; the browser half lives in e2e/notes.spec.ts) ---

#[tokio::test]
async fn index_serves_the_notes_app() {
    let response = app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response.headers()["content-type"].to_str().unwrap();
    assert!(
        content_type.starts_with("text/html"),
        "expected HTML, got {content_type}"
    );

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&body).unwrap();
    // The app, not a placeholder: the list and the form both have to be there.
    assert!(html.contains(r#"data-testid="note-list""#), "no note list");
    assert!(
        html.contains(r#"data-testid="note-input""#),
        "no note input"
    );
}

#[tokio::test]
async fn create_rejects_empty_text() {
    let application = app();

    let rejected = application
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"text":"   "}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(rejected.status(), StatusCode::BAD_REQUEST);

    // Rejected means *not stored* — a 400 that still saved the note would be a lie.
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
    assert_eq!(notes, json!([]));
}

#[tokio::test]
async fn create_trims_surrounding_whitespace() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from("{\"text\":\"  spaced out \\n\"}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let note: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(note["text"], "spaced out");
}
