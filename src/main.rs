mod common;
mod errors;
mod health;
mod routers;
mod user;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::routers::create_app_router;

#[derive(Debug, Deserialize, Clone)]
struct JwtConfig {
    secret: String,
}

#[derive(Clone)]
struct AppState {
    database: DatabaseConnection,
    app_config: AppConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    jwt: JwtConfig,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("dotenv file must be present!");

    let database = Database::connect(
        dotenvy::var("DATABASE_URL").expect("environment variable 'DATABASE_URL' not found!"),
    )
    .await
    .unwrap();

    let jwt_secret =
        dotenvy::var("JWT_SECRET").expect("environment variable 'JWT_SECRET' not found!");

    Migrator::up(&database, None).await.unwrap();

    let app_state = AppState {
        database,
        app_config: AppConfig {
            jwt: JwtConfig { secret: jwt_secret },
        },
    };
    let app_router = create_app_router(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app_router).await.unwrap();
}
