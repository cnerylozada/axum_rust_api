use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api_tags {
    pub const USERS: &str = "USERS";
    pub const TOKENS: &str = "TOKENS";
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "axum101", version= "1.0", description = "My Api description",
        contact(
            name = "Author: Cristian Nery | FullStack web2/web3",
            url = "https://linktr.ee/cnerylozada"
        )
    ),
    tags(
        (name = api_tags::USERS, description = "Users registered by their solana wallet"),
        (name = api_tags::TOKENS, description = "Tokens available in the swap app")
    ),
    paths(
        crate::controllers::users::get_user_list,
        crate::controllers::users::get_user_by_id,
        crate::controllers::users::create_user,
        crate::controllers::tokens::get_token_list,
        crate::controllers::tokens::get_token_by_id
    ),
)]
struct ApiDoc;

pub fn get_api_documentation() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
