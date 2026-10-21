use axum::{
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, errors::ApiError};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JwtClaims {
    sub: String, // subject (user id)
    exp: usize,  // expiration time
    nonce: String,
}

#[derive(Debug, Deserialize)]
struct RefreshTokenPayload {
    refresh_token: String,
}

#[derive(Clone, Debug)]
struct AuthUser {
    id: Uuid,
    name: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<AuthUser>()
            .ok_or(ApiError::InternalServerError)?;

        Ok(user.clone())
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = auth_header
        .ok_or(ApiError::Unauthorized)?
        .token()
        .to_owned();

    let token_data = decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(state.app_config.jwt.secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?;

    let user_id: Uuid = token_data
        .claims
        .sub
        .parse()
        .map_err(|_| ApiError::InternalServerError)?;

    // TODO: fetch user from database
    let user = AuthUser {
        id: user_id,
        name: "User".to_owned(),
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
