use axum::{Json, extract::Path, http::StatusCode};
use crate::models::{User, CreateUser, ApiError, ErrorDetail};

// shared data
fn get_all_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Jimi".to_string() },
        User { id: 2, name: "Alex".to_string() },
    ] 
}

pub async fn get_users() -> Json<Vec<User>> {
    Json(get_all_users())
}

pub async fn get_user(
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

pub async fn create_user(
    Json(payload): Json<CreateUser>
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {

    if payload.name.trim().is_empty() {
        let error = ApiError {
            error: ErrorDetail {
                r#type: "invalid_input".to_string(),
                resource: "user".to_string(),
                id: 0,
            },
        };

        return Err((StatusCode::BAD_REQUEST, Json(error)));
    }

   
    let users = get_all_users();
    let new_id = users.len() as u32 + 1;

    let new_user = User {
        id: new_id,
        name: payload.name,
    };

    Ok(Json(new_user))
}