use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::models::{ApiError, CreateUser, ErrorDetail, UpdateUser, User};
use crate::state::AppState;

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<ApiError>)> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users ORDER BY id")
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;
    Ok(Json(users))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {
    let user = sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(internal_error)?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(not_found(id)),
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {
    if payload.name.trim().is_empty() {
        return Err(invalid_input(0));
    }
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
    )
    .bind(&payload.name)
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {
    if payload.name.trim().is_empty() {
        return Err(invalid_input(id));
    }
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name",
    )
    .bind(&payload.name)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(not_found(id)),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, Json<ApiError>)> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(internal_error)?;
    if result.rows_affected() == 0 {
        return Err(not_found(id));
    }
    Ok(StatusCode::NO_CONTENT)
}

fn not_found(id: i32) -> (StatusCode, Json<ApiError>) {
    (StatusCode::NOT_FOUND, Json(ApiError {
        error: ErrorDetail { r#type: "not_found".into(), resource: "user".into(), id },
    }))
}

fn invalid_input(id: i32) -> (StatusCode, Json<ApiError>) {
    (StatusCode::BAD_REQUEST, Json(ApiError {
        error: ErrorDetail { r#type: "invalid_input".into(), resource: "user".into(), id },
    }))
}

fn internal_error(_: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError {
        error: ErrorDetail { r#type: "internal_error".into(), resource: "user".into(), id: 0 },
    }))
}
