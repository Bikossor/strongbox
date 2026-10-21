mod dto;
mod handlers;

use crate::{AppState, health::boundary::handlers::get_health_check};
use axum::{Router, routing::get};

pub fn get_public_router() -> Router<AppState> {
    Router::new().route("/health", get(get_health_check))
}
