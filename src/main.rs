use axum::Router;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

mod constants;
mod controllers;
mod routes;
use constants::{DEFAULT_ADDRESS, GLOBAL_PREXIF, SUPABASE_SESSION_POOLER};
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

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let pool = database_connection().await.unwrap();

    let app = Router::new().nest(GLOBAL_PREXIF, main_router(pool));

    let listener = tokio::net::TcpListener::bind(DEFAULT_ADDRESS)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
