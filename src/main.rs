use axum::{
    Json, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {}

enum ApiError {
    Conflict,
    InternalServerError,
    NotFound,
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Conflict => (StatusCode::CONFLICT, "Conflict".to_string()),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        let body = Json(json!({ "message": message }));
        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app_state = AppState {};
    let public_router = Router::new()
        .route("/health", get(health_check))
        .route("/register", post(register_user))
        .route("/login", post(login_user));
    let protected_router =
        Router::new()
            .route("/logout", post(logout))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            ));

    let api_router = Router::new().merge(public_router).merge(protected_router);
    let frontend_service = get_service(
        ServeDir::new("frontend/index.html")
            .not_found_service(ServeFile::new("frontend/index.html")),
    );

    let app_router = Router::new()
        .nest("/api/v1", api_router)
        .fallback_service(frontend_service);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app_router).await.unwrap();
}

async fn health_check() {
    todo!()
}

async fn register_user() {
    todo!()
}

async fn login_user() {
    todo!()
}

async fn logout() {
    todo!()
}

async fn auth_middleware(State(state): State<AppState>, request: Request, next: Next) -> Response {
    todo!()
}
