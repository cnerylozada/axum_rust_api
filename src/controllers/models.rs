use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Serialize, FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    age: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreteUserDto {
    pub username: String,
    pub age: i64,
}
