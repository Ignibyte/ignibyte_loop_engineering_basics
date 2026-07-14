//! HTTP handlers — one per endpoint.
//!
//! Handlers stay thin: read the request, call into [`AppState`], return a
//! response. No business logic lives here — validating a note belongs to the
//! model, storing one belongs to the state.

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};

use crate::model::{ApiError, NewNote, Note};
use crate::state::AppState;

/// The notes app, baked into the binary at compile time.
///
/// Embedding it keeps the service a single self-contained binary: no asset
/// directory to ship alongside it, and no static-file middleware to add.
const INDEX_HTML: &str = include_str!("../static/index.html");

/// `GET /` — the notes app: list the notes, add a note.
pub async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// `GET /healthz` — liveness check.
pub async fn healthz() -> &'static str {
    "ok"
}

/// `GET /api/notes` — list every note as JSON.
pub async fn list_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    Json(state.all())
}

/// `POST /api/notes` — add a note.
///
/// Returns `201` with the stored note, `400` if the text is empty, or `500` if
/// the note could not be persisted — in which case nothing is stored.
pub async fn create_note(State(state): State<AppState>, Json(body): Json<NewNote>) -> Response {
    let text = match body.into_text() {
        Ok(text) => text,
        Err(invalid) => {
            let body = ApiError {
                error: invalid.message(),
            };
            return (StatusCode::BAD_REQUEST, Json(body)).into_response();
        }
    };

    match state.add(text) {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(err) => {
            eprintln!("failed to persist note: {err}");
            let body = ApiError {
                error: "could not save the note",
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        }
    }
}
