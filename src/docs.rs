
use crate::models::{
    user::{UserOutput, UserRegister, UserWithRoles},
    auth::{Login, TokenResponse, RefreshTokenInput, Claims},
    role::Role,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // User endpoints
        crate::handlers::user::create_user_handler,
        crate::handlers::user::get_user_handler,
        crate::handlers::user::update_user_handler,
        crate::handlers::user::delete_user_handler,
        // Auth endpoints
        crate::handlers::auth::login_handler,
        crate::handlers::auth::logout_handler,
        crate::handlers::auth::refresh_token_handler,
    ),
    components(
        schemas(
            // User models
            UserRegister,
            UserOutput,
            UserWithRoles,
            // Auth models
            Login,
            TokenResponse,
            RefreshTokenInput,
            Claims,
            // Role models
            Role,
        )
    ),
    tags(
        (name = "rust-auth-service", description = "Rust Auth Service API - Complete authentication and user management system"),
        (name = "users", description = "User management operations"),
        (name = "auth", description = "Authentication operations"),
        (name = "admin", description = "Admin-only operations")
    ),
    info(
        title = "Rust Auth Service API",
        description = "A comprehensive authentication and authorization service built with Rust, Axum, and PostgreSQL",
        version = "1.0.0",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:4000", description = "Local development server"),
        (url = "https://api.example.com", description = "Production server")
    )
)]
pub struct ApiDoc;
