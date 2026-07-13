//! HTTP handlers — one per endpoint.
//!
//! Handlers stay thin: read the request, call into [`AppState`], return a
//! response. No business logic lives here.

use axum::{extract::State, http::StatusCode, response::Html, Json};

use crate::model::{NewNote, Note};
use crate::state::AppState;

/// `GET /` — a tiny HTML index describing the service.
pub async fn index() -> Html<&'static str> {
    Html(
        "<!doctype html><title>ignibyte_loop_engineering_basics</title>\
         <h1>ignibyte_loop_engineering_basics</h1>\
         <p>Demo web app for the Beginner Loop Engineering series.</p>\
         <p>Try <code>GET /api/notes</code> or <code>GET /healthz</code>.</p>",
    )
}

/// `GET /healthz` — liveness check.
pub async fn healthz() -> &'static str {
    "ok"
}

/// `GET /api/notes` — list every note as JSON.
pub async fn list_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    Json(state.all())
}

/// `POST /api/notes` — add a note; returns the created note as JSON with `201`.
pub async fn create_note(
    State(state): State<AppState>,
    Json(body): Json<NewNote>,
) -> (StatusCode, Json<Note>) {
    (StatusCode::CREATED, Json(state.add(body.text)))
}
