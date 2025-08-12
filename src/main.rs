use rust_auth_service::{
    config::{init_pool, init_redis},
    docs::ApiDoc,
    middleware::cors::cors,
    models::app::AppState,
    routes::routes::routes,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let pool = init_pool().await;
    let redis = init_redis().await;

    let app_state = AppState { pool, redis };

    let app = routes(&app_state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Listening on http://localhost:4000");

    axum::serve(listener, app).await.unwrap();
}
