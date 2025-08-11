use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use redis::AsyncCommands;
use std::net::IpAddr;

use crate::{errors::my_error::MyError, models::app::AppState};

pub async fn rate_limit_middleware(
    State(app_state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, MyError> {
    let mut redis_conn = app_state.redis;
    let headers = request.headers();

    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|ip| ip.to_str().ok())
        .and_then(|ip| ip.split(',').next().map(|s| s.trim()))
        .filter(|ip| ip.parse::<IpAddr>().is_ok())
        .or_else(|| headers.get("x-real-ip").and_then(|ip| ip.to_str().ok()).filter(|ip| ip.parse::<IpAddr>().is_ok()))
        .or_else(|| headers.get("cf-connecting-ip").and_then(|ip| ip.to_str().ok()).filter(|ip| ip.parse::<IpAddr>().is_ok()))
        .unwrap_or("unknown")
        .to_string();

    println!("Client IP: {}", &client_ip);

    let key = format!("rate_limit:{}:{}", client_ip, request.uri().path());

    println!("Rate limit key: {}", key);

    let limit: i32 = redis_conn.incr(&key, 1).await.unwrap();

    println!("Rate limit: {}", limit);

    if limit == 1 {
        redis_conn.expire::<_, ()>(&key, 60).await.unwrap();
    }

    if limit > 10 {
        return Err(MyError::TooManyRequests);
    }

    println!("Rate limit: {}", limit);

    Ok(next.run(request).await)
}
