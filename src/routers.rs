use crate::{AppState, common, health, user};
use axum::{
    Router,
    middleware::{self},
    routing::get_service,
};
use tower_http::services::{ServeDir, ServeFile};

pub fn create_app_router(app_state: AppState) -> Router {
    let public_router = Router::new()
        .merge(health::boundary::get_public_router())
        .merge(user::boundary::get_public_router());

    let protected_router = Router::new()
        .merge(user::boundary::get_private_router())
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            common::auth::auth_middleware,
        ));

    let api_router = Router::new()
        .merge(public_router)
        .merge(protected_router)
        .with_state(app_state.clone());

    let frontend_service = get_service(
        ServeDir::new("frontend/index.html")
            .not_found_service(ServeFile::new("frontend/index.html")),
    );

    Router::new()
        .nest("/api/v1", api_router)
        .fallback_service(frontend_service)
}
