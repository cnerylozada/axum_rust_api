use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::controllers::users::get_user_list,crate::controllers::users::get_user_by_id),
    info(title = "axum101", description = "My Api description"),
    tags((name = "USERS", description = "Some description"))
)]
struct ApiDoc;

pub fn get_api_documentation() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
