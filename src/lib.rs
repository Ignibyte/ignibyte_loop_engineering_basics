//! Demo web app for the Beginner Loop Engineering series.
//!
//! The library target builds the axum [`Router`] via [`app`]/[`app_with_state`];
//! the binary target (`main.rs`) just binds a port and serves it. Splitting it
//! this way lets the integration tests in `tests/` exercise the router without
//! opening a socket.

#![forbid(unsafe_code)]

pub mod model;
pub mod routes;
pub mod state;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

/// Build the application router around a given [`AppState`].
pub fn app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index))
        .route("/healthz", get(routes::healthz))
        .route("/api/notes", get(routes::list_notes))
        .route("/api/notes", post(routes::create_note))
        .with_state(state)
}

/// Build the application router with a fresh in-memory store (no persistence).
pub fn app() -> Router {
    app_with_state(AppState::new(None))
}
