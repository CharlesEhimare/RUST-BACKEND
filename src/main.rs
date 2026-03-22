use axum::{routing::get, Router};
use serde::Serialize;
use axum::extract::Path;
#[derive(Serialize)]

struct User {
    id: u32,
    name: String,
} 
use axum::Json;

async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: "Jimi".to_string() },
        User { id: 2, name: "Alex".to_string() },
    ];

    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/users/:id", get(get_users));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Server is running"
}
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    let user  = User{
        id,
        name: format!("User {}", id),   
    };

    Json(user)
}