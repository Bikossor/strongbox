mod errors;

use axum::{
    Json, Router,
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::{get, get_service, post},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

struct JwtClaims {
    sub: String, // subject (user id)
    exp: usize,  // expiration time
    nonce: String,
}

#[derive(Debug, Deserialize)]
struct RefreshTokenPayload {
    refresh_token: String,
}

#[derive(Clone)]
struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("dotenv file must be present!");

    let database = Database::connect(
        dotenvy::var("DATABASE_URL").expect("environment variable 'DATABASE_URL' not found!"),
    )
    .await
    .unwrap();

    Migrator::up(&database, None).await.unwrap();

    let app_state = AppState { database };

    let public_router = Router::new()
        .route("/health", get(get_health_check))
        .route("/register", post(post_register_user))
        .route("/login", post(post_login_user));
    let protected_router =
        Router::new()
            .route("/logout", post(logout))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            ));

    let api_router = Router::new()
        .merge(public_router)
        .merge(protected_router)
        .with_state(app_state.clone());
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

#[derive(Serialize)]
struct HealthCheckResponse {
    pub database_healthy: bool,
}

async fn get_health_check(State(state): State<AppState>) -> Json<HealthCheckResponse> {
    return match state.database.ping().await {
        Ok(_) => Json(HealthCheckResponse {
            database_healthy: true,
        }),
        Err(_) => Json(HealthCheckResponse {
            database_healthy: false,
        }),
    };
}

async fn post_register_user() {
    todo!()
}

async fn post_login_user() {
    todo!()
}

async fn logout() {
    todo!()
}

async fn auth_middleware(State(state): State<AppState>, request: Request, next: Next) -> Response {
    todo!()
}
