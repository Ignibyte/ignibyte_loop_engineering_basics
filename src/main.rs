//! Entry point: bind a TCP port and serve the router built in the library.

use ignibyte_loop_engineering_basics::app;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind 127.0.0.1:3000");
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app()).await.expect("server error");
}
