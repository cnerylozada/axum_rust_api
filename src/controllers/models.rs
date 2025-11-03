use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};
use utoipa::ToSchema;

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    pub wallet: Option<String>,
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct User {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    id: Uuid,
    #[schema(example = "AKeJdxqP6MpFyhcFGUN79NTUwe2ntZNoGjw37UTbbFp")]
    wallet: String,
    #[schema(example = "cnerylozada@gmail.com")]
    email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreteUserDto {
    #[schema(example = "AKeJdxqP6MpFyhcFGUN79NTUwe2ntZNoGjw37UTbbFp")]
    pub wallet: i64,
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct Token {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "mntXmMnUP9vJYxbfykG2ZQhgcFHth6kwg8sVJTBY1pX")]
    pub mint_address: String,
}
