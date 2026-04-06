use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
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
    pub id: u32,
}