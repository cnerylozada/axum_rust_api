use axum::Router;
use std::env;

mod controllers;
mod routes;
use routes::main_router;

const GLOBAL_PREXIF: &str = "/api/v1";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_connection = env::var("SUPABASE_CONNECTION").expect("Can not read env variable");
    println!("database_connection {}", database_connection);

    let app = Router::new().nest(GLOBAL_PREXIF, main_router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
