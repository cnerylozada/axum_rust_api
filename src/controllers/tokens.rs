use crate::{controllers::models::Token, documentation::api_tags};
use axum::Json;
use sqlx::types::Uuid;

#[utoipa::path(
    tag = api_tags::TOKENS,
    get,
    path = "/api/v1/tokens",
    responses(
        (status = 200, description = "All available tokens", body = [Token])
    )
)]
pub async fn get_token_list() -> Result<Json<Vec<Token>>, String> {
    let response = vec![Token {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        mint_address: String::from("mntXmMnUP9vJYxbfykG2ZQhgcFHth6kwg8sVJTBY1pX"),
    }];
    Ok(Json(response))
}
