use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use headers::{ Authorization, HeaderMapExt, authorization::Bearer };
use std::future::Future;

use crate::{ auth::jwt::decode_access_token, errors::my_error::MyError, models::auth::Claims };

// middeware Extractor Pattern Axum
impl<S> FromRequestParts<S> for Claims where S: Send + Sync {
    type Rejection = MyError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let auth_header = parts.headers
                .typed_get::<Authorization<Bearer>>()
                .ok_or(MyError::Unauthorized)?;

            let token = auth_header.token();
            let claims = decode_access_token(token)?;

            Ok(claims)
        }
    }
}
