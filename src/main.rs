use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Define a simple GET route
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    // Bind to port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    // Start service
    axum::serve(listener, app).await.unwrap(); // ← if the port is already in use, this panics
}