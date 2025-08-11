use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::request::{Parts, Request},
    middleware::Next,
    response::Response,
};

use headers::{Authorization, HeaderMapExt, authorization::Bearer};
use std::future::Future;

use crate::{
    auth::auth::{decode_access_token, validate_jwt},
    errors::my_error::MyError,
    models::{app::AppState, auth::Claims},
};

// middeware Extractor Pattern Axum
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = MyError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let auth_header = parts
                .headers
                .typed_get::<Authorization<Bearer>>()
                .ok_or(MyError::Unauthorized)?;

            let token = auth_header.token();
            let claims = decode_access_token(token)?;

            Ok(claims)
        }
    }
}

// common middleware
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, MyError> {
    let started_at = std::time::Instant::now();
    let mut redis_conn = app_state.redis;
    let auth_header = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(MyError::Unauthorized)?;

    let token = auth_header.token();

    let claims = validate_jwt(&mut redis_conn, token).await?;

    request.extensions_mut().insert(claims);

    println!("Request took {} ms", started_at.elapsed().as_millis());

    Ok(next.run(request).await)
}
