use rust_auth_service::db::role::{get_role_by_name, set_user_role};
use sqlx::PgPool;
use rust_auth_service::db::user::{create_user, get_user_by_email};
use rust_auth_service::models::user::UserRegister;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    
    println!("Connected to database successfully");
    
    let admin_user = UserRegister {
        name: Some("Admin".to_string()),
        email: Some("admin@example.com".to_string()),
        password: Some("admin123".to_string()),
    };
    
    let exist_admin_user = get_user_by_email(&pool, "admin@example.com".to_string()).await?;

    if exist_admin_user.is_some() {
        println!("Admin user already exists");
        return Ok(());
    }

    let admin_user = create_user(&pool, admin_user).await?;
    println!("Admin user created: {}", admin_user.name);

    let admin_role = get_role_by_name(&pool, "Admin".to_string()).await?;
    println!("Admin role: {}", admin_role.name);

    let _ = set_user_role(&pool, admin_user.id, admin_role.id).await?;
    println!("User role assigned successfully");

    println!("Seed data created successfully!");
    println!("Admin user: admin@example.com / admin123");
    
    Ok(())
}
