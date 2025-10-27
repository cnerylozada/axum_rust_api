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

#[derive(Debug, Deserialize)]
pub struct CreteUserDto {
    pub username: String,
    pub age: i64,
}
