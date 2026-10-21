use super::dto::HealthCheckResponse;
use crate::AppState;

use axum::{Json, extract::State};
use chrono::Utc;

pub async fn get_health_check(State(state): State<AppState>) -> Json<HealthCheckResponse> {
    return match state.database.ping().await {
        Ok(_) => Json(HealthCheckResponse {
            database_healthy: true,
            timestamp: Utc::now(),
        }),
        Err(_) => Json(HealthCheckResponse {
            database_healthy: false,
            timestamp: Utc::now(),
        }),
    };
}
