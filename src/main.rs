use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app_router = Router::new().fallback(hello_world);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app_router).await.unwrap();
}

// basic handler that responds with a static string
async fn hello_world() -> &'static str {
    "Hello, world!"
}
