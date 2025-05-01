use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::handlers::user::{
    create_user_handler, delete_user_handler, get_user_handler, update_user_handler,
};

pub fn routes() -> Router {
    let root_router = Router::new().route("/", get(|| async { "Hello, World!" }));

    // TODO: try to implement one instance of pool db and reuse it

    let user_router = Router::new()
        .route("/users/{user_id}", get(get_user_handler))
        .route("/users", post(create_user_handler))
        .route("/users/{user_id}", patch(update_user_handler))
        .route("/users/{user_id}", delete(delete_user_handler));

    let app_routes = Router::new().merge(root_router).merge(user_router);
    let app = Router::new().nest("/api", app_routes);
    app
}
