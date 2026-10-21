use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub database_healthy: bool,
    pub timestamp: DateTime<Utc>,
}
