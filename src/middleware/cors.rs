use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // configure allowed origins
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
}
