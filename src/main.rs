use rust_auth_service::{ config::init_pool, middleware::cors::cors, routes::routes::routes };

#[tokio::main]
async fn main() {
    let pool = init_pool().await;

    let app = routes().layer(cors()).with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();

    println!("Listening on http://localhost:4000");

    axum::serve(listener, app).await.unwrap();
}
