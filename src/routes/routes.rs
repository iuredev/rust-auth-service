use crate::{
    handlers::{
        auth::{login_handler, logout_handler, refresh_token_handler},
        user::{create_user_handler, delete_user_handler, get_user_handler, update_user_handler},
    },
    middleware::{auth::auth_middleware, rate_limit::rate_limit_middleware},
    models::app::AppState,
};
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, patch, post},
};

pub fn routes(state: &AppState) -> Router<AppState> {
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
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .layer(from_fn_with_state(state.clone(), rate_limit_middleware));

    let refresh = Router::new().route("/refresh", post(refresh_token_handler));

    let app_routes = Router::new()
        .merge(root_router)
        .merge(user_router)
        .merge(refresh)
        .merge(protected);

    let app = Router::new().nest("/api", app_routes);

    app
}
