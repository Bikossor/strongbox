mod errors;
mod routers;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::routers::create_app_router;

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
    let app_router = create_app_router(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app_router).await.unwrap();
}
