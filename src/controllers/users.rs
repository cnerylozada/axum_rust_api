use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::{PgPool, types::Uuid};

use crate::{
    controllers::models::{CreteUserDto, User},
    models::ApiErrorResponse,
};

pub async fn get_user_list(
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, ApiErrorResponse)> {
    let query = "SELECT * FROM users";

    let user_list = sqlx::query_as::<_, User>(query)
        .fetch_all(&db_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse {
                    message: error.to_string(),
                },
            )
        })?;

    Ok(Json(user_list))
}

pub async fn get_user_by_id(
    Path(user_id): Path<String>,
    State(db_pool): State<PgPool>,
) -> Result<Json<User>, (StatusCode, ApiErrorResponse)> {
    let id = Uuid::parse_str(&user_id).map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            ApiErrorResponse {
                message: error.to_string(),
            },
        )
    })?;

    let query = "SELECT * FROM users WHERE id = $1";
    let user_response = sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(&db_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::NOT_FOUND,
                ApiErrorResponse {
                    message: error.to_string(),
                },
            )
        })?;
    Ok(Json(user_response))
}

pub async fn create_user(
    State(db_pool): State<PgPool>,
    Json(user_dto): Json<CreteUserDto>,
) -> Result<Json<User>, (StatusCode, ApiErrorResponse)> {
    println!("user_dto: {:?}", user_dto);

    Ok(Json(User {
        id: Uuid::parse_str("eff1c0b3-b749-448e-9c9e-2bc291e5a232").unwrap(),
        username: String::from("lucciano"),
        age: 5,
    }))
}
