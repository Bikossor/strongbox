mod handlers;

use axum::{Router, routing::post};

use crate::{
    AppState,
    user::boundary::handlers::{logout, post_login_user, post_register_user},
};

pub fn get_public_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(post_register_user))
        .route("/login", post(post_login_user))
}

pub fn get_private_router() -> Router<AppState> {
    Router::new().route("/logout", post(logout))
}
