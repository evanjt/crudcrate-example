mod todo;

use axum::Router;
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

pub fn build_router(db: &DatabaseConnection) -> Router {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    // Build the router with routes from the plots module
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(crate::common::views::router(db)) // Root routes
        .nest("/todo", todo::views::router(db))
        .split_for_parts();

    router.merge(Scalar::with_url("/docs", api))
}
