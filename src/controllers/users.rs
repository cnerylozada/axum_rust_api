use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    username: String,
    age: i64,
}

pub async fn get_user_list(State(db_pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>(r#"SELECT username, age FROM "USERS""#)
        .fetch_all(&db_pool)
        .await
        .unwrap();

    Json(users)
}

pub async fn get_user_by_id(Path(user_id): Path<String>) -> Json<User> {
    println!("get_user_by_id user_id: {}", user_id);

    let user = User {
        username: "cnerylozada".to_string(),
        age: 32,
    };
    Json(user)
}
