use rust_auth_service::{middleware::cors::cors, routes::routes::routes};

#[tokio::main]
async fn main() {
    let app = routes().layer(cors());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
