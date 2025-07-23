use crate::{
    handlers::{
        auth::{login_handler, logout_handler},
        user::{create_user_handler, delete_user_handler, get_user_handler, update_user_handler},
    },
    middleware::auth::auth_middleware,
};
use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, patch, post},
};

pub fn routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    let root_router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let user_router = Router::new()
        .route("/users/{user_id}", get(get_user_handler))
        .route("/users", post(create_user_handler))
        .route("/users/{user_id}", patch(update_user_handler))
        .route("/users/{user_id}", delete(delete_user_handler))
        .route("/login", post(login_handler));

    // protected router
    let protected = Router::new()
        .route("/logout", post(logout_handler))
        .layer(from_fn(auth_middleware));

    let app_routes = Router::new()
        .merge(root_router)
        .merge(user_router)
        .merge(protected);
    let app = Router::new().nest("/api", app_routes);
    app
}
