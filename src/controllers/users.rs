use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    username: String,
    age: i64,
}

#[derive(Serialize)]
pub struct ApiError {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub async fn get_user_list(
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, ApiError)> {
    let usere_list_response = sqlx::query_as::<_, User>(r#"SELECT username, age FROM "USERS""#)
        .fetch_all(&db_pool)
        .await;

    match usere_list_response {
        Ok(user_list) => Ok(Json(user_list)),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiError {
                message: format!("Database error: {}", error),
            },
        )),
    }
}

pub async fn get_user_by_id(Path(user_id): Path<String>) -> Json<User> {
    println!("get_user_by_id user_id: {}", user_id);

    let user = User {
        username: "cnerylozada".to_string(),
        age: 32,
    };
    Json(user)
}
