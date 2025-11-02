use crate::controllers::tokens::get_token_list;
use axum::{Router, routing::get};

pub fn tokens_routes() -> Router {
    Router::new().route("/", get(get_token_list))
}
