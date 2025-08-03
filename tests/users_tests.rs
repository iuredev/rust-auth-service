use rust_auth_service::models::user::{ User };
use uuid::Uuid;

#[test]
fn should_create_user_with_valid_data() {
    let user = User::new(
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john.doe@example.com");
    // Password should be hashed, not plain text
    assert_ne!(user.password, "password123");
    assert!(user.password.len() > 20); // Argon2 hash is long
    assert!(Uuid::parse_str(&user.id.to_string()).is_ok());
}

#[test]
fn should_trim_whitespace_from_user_data() {
    let user = User::new(
        "  John Doe  ".to_string(),
        "  JOHN.DOE@EXAMPLE.COM  ".to_string(),
        "  password123  ".to_string()
    );

    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john.doe@example.com"); // Should be lowercase
    assert_ne!(user.password, "  password123  ");
}

#[test]
fn should_generate_unique_ids_for_different_users() {
    let user1 = User::new(
        "User 1".to_string(),
        "user1@example.com".to_string(),
        "password123".to_string()
    );

    let user2 = User::new(
        "User 2".to_string(),
        "user2@example.com".to_string(),
        "password123".to_string()
    );

    assert_ne!(user1.id, user2.id);
    assert_ne!(user1.created_at, user2.created_at);
}

#[test]
fn should_set_timestamps_on_user_creation() {
    let before_creation = chrono::Utc::now();

    let user = User::new(
        "Test User".to_string(),
        "test@example.com".to_string(),
        "password123".to_string()
    );

    let after_creation = chrono::Utc::now();

    assert!(user.created_at >= before_creation);
    assert!(user.created_at <= after_creation);
    assert!(user.updated_at >= before_creation);
    assert!(user.updated_at <= after_creation);
    assert_eq!(user.created_at, user.updated_at); // Should be equal on creation
}

#[test]
fn should_handle_empty_strings_in_user_creation() {
    let user = User::new("".to_string(), "".to_string(), "".to_string());

    assert_eq!(user.name, "");
    assert_eq!(user.email, "");
    assert_ne!(user.password, ""); // Password should still be hashed
}

#[test]
fn should_handle_special_characters_in_name() {
    let user = User::new(
        "João Silva-Santos".to_string(),
        "joao.silva@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.name, "João Silva-Santos");
    assert_eq!(user.email, "joao.silva@example.com");
}

#[test]
fn should_handle_long_email_addresses() {
    let long_email = format!(
        "very.long.email.address.with.many.subdomains.{}@very.long.domain.example.com",
        Uuid::new_v4()
    );

    let user = User::new("Test User".to_string(), long_email.clone(), "password123".to_string());

    assert_eq!(user.email, long_email.to_lowercase());
}

#[test]
fn should_handle_unicode_characters() {
    let user = User::new(
        "José María García-López".to_string(),
        "josé.maría@españa.es".to_string(),
        "contraseña123".to_string()
    );

    assert_eq!(user.name, "José María García-López");
    assert_eq!(user.email, "josé.maría@españa.es");
}

#[test]
fn should_handle_numbers_in_name_and_email() {
    let user = User::new(
        "User123".to_string(),
        "user123@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.name, "User123");
    assert_eq!(user.email, "user123@example.com");
}

#[test]
fn should_handle_very_long_names() {
    let long_name = "A".repeat(1000);

    let user = User::new(
        long_name.clone(),
        "test@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.name, long_name);
}

#[test]
fn should_handle_very_long_passwords() {
    let long_password = "A".repeat(1000);

    let user = User::new(
        "Test User".to_string(),
        "test@example.com".to_string(),
        long_password.clone()
    );

    assert_ne!(user.password, long_password); // Should be hashed
    assert!(user.password.len() > 20); // Hash should be long
}

#[test]
fn should_handle_special_characters_in_password() {
    let special_password = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

    let user = User::new(
        "Test User".to_string(),
        "test@example.com".to_string(),
        special_password.to_string()
    );

    assert_ne!(user.password, special_password); // Should be hashed
    assert!(user.password.len() > 20); // Hash should be long
}

#[test]
fn should_handle_mixed_case_email() {
    let user = User::new(
        "Test User".to_string(),
        "TEST.USER@EXAMPLE.COM".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test.user@example.com"); // Should be lowercase
}

#[test]
fn should_handle_email_with_plus_sign() {
    let user = User::new(
        "Test User".to_string(),
        "test+tag@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test+tag@example.com");
}

#[test]
fn should_handle_email_with_dots() {
    let user = User::new(
        "Test User".to_string(),
        "test.user.name@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test.user.name@example.com");
}

#[test]
fn should_handle_email_with_hyphens() {
    let user = User::new(
        "Test User".to_string(),
        "test-user@example-domain.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test-user@example-domain.com");
}

#[test]
fn should_handle_email_with_underscores() {
    let user = User::new(
        "Test User".to_string(),
        "test_user@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test_user@example.com");
}

#[test]
fn should_handle_email_with_numbers() {
    let user = User::new(
        "Test User".to_string(),
        "test123@example456.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test123@example456.com");
}

#[test]
fn should_handle_email_with_multiple_at_signs_in_local_part() {
    // This should be handled gracefully even though it's not a valid email
    let user = User::new(
        "Test User".to_string(),
        "test@user@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test@user@example.com");
}

#[test]
fn should_handle_email_with_spaces() {
    let user = User::new(
        "Test User".to_string(),
        "  test@example.com  ".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.email, "test@example.com"); // Should be trimmed and lowercase
}

#[test]
fn should_handle_name_with_spaces() {
    let user = User::new(
        "  John   Doe  ".to_string(),
        "test@example.com".to_string(),
        "password123".to_string()
    );

    assert_eq!(user.name, "John   Doe"); // Should trim but preserve internal spaces
}
