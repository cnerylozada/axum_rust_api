use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow, types::Uuid};

use crate::controllers::models::ApiErrorResponse;

#[derive(Serialize, FromRow)]
pub struct User {
    username: String,
    age: i64,
}

pub async fn get_user_list(
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, ApiErrorResponse)> {
    let query = "SELECT username, age FROM users";
    let usere_list_response = sqlx::query_as::<_, User>(query).fetch_all(&db_pool).await;

    match usere_list_response {
        Ok(user_list) => Ok(Json(user_list)),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorResponse {
                message: error.to_string(),
            },
        )),
    }
}

pub async fn get_user_by_id(
    Path(user_id): Path<String>,
    State(db_pool): State<PgPool>,
) -> Result<Json<User>, (StatusCode, ApiErrorResponse)> {
    let id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse {
                    message: error.to_string(),
                },
            ));
        }
    };

    let query = "SELECT username, age FROM users WHERE id = $1";
    let user_response = sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(&db_pool)
        .await;

    match user_response {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            ApiErrorResponse {
                message: error.to_string(),
            },
        )),
    }
}
