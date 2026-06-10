use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: String,
}

#[derive(Serialize)]
pub struct ApiError {
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub r#type: String,
    pub resource: String,
    pub id: i32,
}
