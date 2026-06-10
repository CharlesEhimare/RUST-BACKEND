mod handler;
mod models;
mod state;

use axum::{routing::get, Router};
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let db = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let state = AppState::new(db);

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(handler::get_users).post(handler::create_user))
        .route(
            "/users/:id",
            get(handler::get_user)
                .put(handler::update_user)
                .delete(handler::delete_user),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Server is running"
}
