//! Entry point: bind a TCP port and serve the router built in the library.

#![forbid(unsafe_code)]

use std::path::PathBuf;

use ignibyte_loop_engineering_basics::{app_with_state, state::AppState};

#[tokio::main]
async fn main() {
    // Notes persist to this file across restarts (override with NOTES_FILE).
    let path = std::env::var_os("NOTES_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("notes.json"));
    let state = AppState::new(Some(path));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind 127.0.0.1:3000");
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app_with_state(state))
        .await
        .expect("server error");
}
