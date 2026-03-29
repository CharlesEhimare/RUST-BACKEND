use axum::{routing::{get, post}, Router, Json, extract::Path, http::StatusCode};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Serialize)]
struct ApiError {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    r#type: String,
    resource: String,
    id: u32,
}


fn get_all_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Jimi".to_string() },
        User { id: 2, name: "Alex".to_string() },
    ]
}


async fn get_users() -> Json<Vec<User>> {
    Json(get_all_users())
}


async fn get_user(
    Path(id): Path<u32>,
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {
    let users = get_all_users();

    for user in users {
        if user.id == id {
            return Ok(Json(user));
        }
    }

    let error = ApiError {
        error: ErrorDetail {
            r#type: "not_found".to_string(),
            resource: "user".to_string(),
            id,
        },
    };

    Err((StatusCode::NOT_FOUND, Json(error)))
}


async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let new_user = User {
        id: 3,
        name: payload.name,
    };

    Json(new_user)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Server is running"
}