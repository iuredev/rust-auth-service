use crate::{
    handlers::{
        auth::{login_handler, logout_handler, refresh_token_handler},
        user::{create_user_handler, delete_user_handler, get_user_handler, update_user_handler},
    },
    middleware::{auth::{auth_middleware, require_role}, rate_limit::rate_limit_middleware},
    models::app::AppState,
};
use axum::{
    Router,
    middleware::{from_fn,from_fn_with_state},
    routing::{delete, get, patch, post},
};

pub fn routes(state: &AppState) -> Router<AppState> {
    let root_router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let public = Router::new()
        .route("/refresh", post(refresh_token_handler))
        .route("/users", post(create_user_handler))
        .route("/login", post(login_handler));

    let protected = Router::new()
        .route("/logout", post(logout_handler))
        // Users
        .route("/users/{user_id}", get(get_user_handler))
        .route("/users/{user_id}", patch(update_user_handler))
        .route("/users/{user_id}", delete(delete_user_handler))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .layer(from_fn_with_state(state.clone(), rate_limit_middleware));

    let admin_router = Router::new()
        .route("/admin", get(|| async { "Route only for Admin" }))
        .layer(from_fn(require_role(vec!["Admin".to_string()])))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    let app_routes = Router::new()
        .merge(admin_router)
        .merge(root_router)
        .merge(public)
        .merge(protected);


    let app = Router::new().nest("/api", app_routes);

    app
}
