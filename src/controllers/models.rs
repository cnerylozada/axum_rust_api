use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};
use utoipa::ToSchema;

#[derive(Serialize, FromRow, ToSchema)]
pub struct User {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    id: Uuid,
    #[schema(example = "cnerylozada")]
    username: String,
    #[schema(example = 32)]
    age: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreteUserDto {
    #[schema(example = "devgrim07")]
    pub username: String,
    #[schema(example = 32)]
    pub age: i64,
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct Token {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "mntXmMnUP9vJYxbfykG2ZQhgcFHth6kwg8sVJTBY1pX")]
    pub mint_address: String,
}
