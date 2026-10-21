use crate::{AppState, JwtClaims, errors::ApiError, health, user};
use axum::{
    Router,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::{self, Next},
    response::Response,
    routing::get_service,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

pub fn create_app_router(app_state: AppState) -> Router {
    let public_router = Router::new()
        .merge(health::boundary::get_public_router())
        .merge(user::boundary::get_public_router());

    let protected_router = Router::new()
        .merge(user::boundary::get_private_router())
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

    Router::new()
        .nest("/api/v1", api_router)
        .fallback_service(frontend_service)
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: Uuid,
    pub name: String,
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

async fn auth_middleware(
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
