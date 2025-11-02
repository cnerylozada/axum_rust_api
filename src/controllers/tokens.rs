use crate::{controllers::models::Token, documentation::api_tags, models::ApiErrorResponse};
use axum::{
    Json,
    extract::{Path, State},
};
use reqwest::StatusCode;
use sqlx::{PgPool, types::Uuid};

#[utoipa::path(
    tag = api_tags::TOKENS,
    get,
    path = "/api/v1/tokens",
    responses(
        (status = 200, description = "All available tokens", body = [Token])
    )
)]
pub async fn get_token_list(
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<Token>>, (StatusCode, ApiErrorResponse)> {
    let query = "SELECT * FROM tokens";

    let token_list = sqlx::query_as::<_, Token>(query)
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

    Ok(Json(token_list))
}

#[utoipa::path(
    tag = api_tags::TOKENS,
    get,
    path = "/api/v1/tokens/{token_id}",
    params(
        ("token_id" = String, Path, description = "Token database id to get Token for")
    ),
    responses(
        (status = 200, description = "Token found successfully", body = Token)
    )

)]
pub async fn get_token_by_id(
    State(db_pool): State<PgPool>,
    Path(token_id): Path<String>,
) -> Result<Json<Token>, (StatusCode, ApiErrorResponse)> {
    let id = Uuid::parse_str(&token_id).map_err(|error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorResponse {
                message: error.to_string(),
            },
        )
    })?;

    let query = "SELECT * FROM tokens where id = $1";
    let token = sqlx::query_as::<_, Token>(query)
        .bind(id)
        .fetch_one(&db_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse {
                    message: error.to_string(),
                },
            )
        })?;

    Ok(Json(token))
}
