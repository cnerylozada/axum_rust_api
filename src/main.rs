use axum::Router;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

mod controllers;
mod routes;
use routes::main_router;

const GLOBAL_PREXIF: &str = "/api/v1";

async fn database_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url =
        env::var("SUPABASE_SESSION_POOLER").expect("SUPABASE_SESSION_POOLER in not found");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;
    Ok(pool)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = database_connection().await.unwrap();

    let app = Router::new().nest(GLOBAL_PREXIF, main_router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
