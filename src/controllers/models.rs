use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub age: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreteUserDto {
    pub username: String,
    pub age: i64,
}
