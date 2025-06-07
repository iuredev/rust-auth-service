use rust_auth_service::models::user::User;

#[test]
fn should_create_user() {
    let user = User::new(
        "John Doe".to_string(),
        "V2A2b@example.com".to_string(),
        "password123".to_string(),
    );

    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "V2A2b@example.com");
    assert_eq!(user.password, "password123");
}
