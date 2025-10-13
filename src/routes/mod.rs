use axum::Router;
mod users;
use users::user_routes;

pub fn main_router() -> Router {
    Router::new().merge(user_routes())
}
