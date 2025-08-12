use rust_auth_service::{
    config::{init_pool, init_redis},
    docs::ApiDoc,
    middleware::cors::cors,
    models::app::AppState,
    routes::routes::routes,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_auth_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rust Auth Service...");

    let pool = init_pool().await;
    let redis = init_redis().await;

    tracing::info!("Pool initialized");
    tracing::info!("Redis initialized");

    let app_state = AppState { pool, redis };

    let app = routes(&app_state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    tracing::info!("Listening on http://localhost:4000");

    axum::serve(listener, app).await.unwrap();
}
