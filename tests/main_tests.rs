#[cfg(test)]
mod tests {
    use axum::{Router, http::Request, routing::post};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new().route("/health", post(|| async { "ok" }));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .method("POST")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
