mod models;
mod handler;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/users", get(handler::get_users).post(handler::create_user))
    .route("/users/:id", get(handler::get_user));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Server is running"
}