use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let public_router = Router::new()
        .route("/health", get(health_check))
        .route("/register", post(register_user))
        .route("/login", post(login_user));
    let protected_router = Router::new().route("/logout", post(logout));

    let api_router = Router::new().merge(public_router).merge(protected_router);

    let app_router = Router::new()
        .nest("/api/v1", api_router)
        .fallback(hello_world);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app_router).await.unwrap();
}

// basic handler that responds with a static string
async fn hello_world() -> &'static str {
    "Hello, world!"
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
