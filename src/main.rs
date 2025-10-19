use axum::Router;
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres, postgres::PgPoolOptions};
use std::env;

mod constants;
mod controllers;
mod routes;
use constants::{GLOBAL_PREXIF, SUPABASE_SESSION_POOLER};
use routes::main_router;

async fn database_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var(SUPABASE_SESSION_POOLER)
        .expect(format!("{} env is not found", SUPABASE_SESSION_POOLER).as_str());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;
    Ok(pool)
}

#[derive(Debug, Serialize, FromRow)]
struct Userx {
    name: String,
    age: i64,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = database_connection().await.unwrap();

    let users = sqlx::query_as::<_, Userx>(r#"SELECT name, age FROM "USERS""#)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("Users from database: {:?}", users);

    let app = Router::new().nest(GLOBAL_PREXIF, main_router());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
