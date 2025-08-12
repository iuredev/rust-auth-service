use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::request::{Parts, Request},
    middleware::Next,
    response::Response,
};

use headers::{Authorization, HeaderMapExt, authorization::Bearer};
use tracing::{info, instrument};
use std::{future::Future, pin::Pin};
use crate::{
    auth::auth::{decode_access_token, validate_jwt},
    errors::my_error::MyError,
    models::{app::AppState, auth::Claims},
};

// Middleware Extractor Pattern Axum (you can se how to use it in the logout_handler that is commented)
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

#[instrument(skip(app_state, request, next))]
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, MyError> {
    let started_at = std::time::Instant::now();
    
    info!("Processing authenticated request to: {}", request.uri());
    
    let mut redis_conn = app_state.redis;
    let auth_header = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(MyError::Unauthorized)?;

    let token = auth_header.token();

    let claims = validate_jwt(&mut redis_conn, token).await?;

    request.extensions_mut().insert(claims);

    let duration = started_at.elapsed();
    tracing::info!("Request processed in {} ms", duration.as_millis());

    Ok(next.run(request).await)
}

pub fn require_role(
    required_roles: Vec<String>,
) -> impl Clone + Send + Sync + 'static + Fn(Request<Body>, Next) -> Pin<Box<dyn Future<Output = Result<Response<Body>, MyError>> + Send>>
{
    move | req: Request<Body>, next: Next| {
        let required_roles = required_roles.clone();

        Box::pin(async move {
            let claims = req.extensions().get::<Claims>().cloned().ok_or(MyError::Unauthorized)?;
            let has_role = claims.roles.iter().any(|role| required_roles.contains(role));

            if !has_role {
                return Err(MyError::Unauthorized);
            }

            Ok(next.run(req).await)
        })
    }
}
