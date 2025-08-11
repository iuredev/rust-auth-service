use rust_auth_service::{
    auth::auth::{decode_access_token, generate_tokens},
    models::{
        auth::{Claims, Login, TokenType},
        user::User,
    },
    services::password::{hash_password, verify_password},
};
use uuid::Uuid;

#[test]
fn should_hash_password_correctly() {
    let password = "my_password_123";
    let hashed = hash_password(password).unwrap();

    assert_ne!(hashed, password);

    assert!(hashed.len() > 50);

    assert!(hashed.starts_with("$argon2id$"));
}

#[test]
fn should_verify_password_correctly() {
    let password = "my_password_123";
    let hashed = hash_password(password).unwrap();

    let is_valid = verify_password(&hashed, password).unwrap();
    assert!(is_valid);

    let is_invalid = verify_password(&hashed, "wrong_password").unwrap();
    assert!(!is_invalid);
}

#[test]
fn should_verify_password_with_special_characters() {
    let password = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let hashed = hash_password(password).unwrap();

    let is_valid = verify_password(&hashed, password).unwrap();
    assert!(is_valid);
}

#[test]
fn should_verify_password_with_unicode() {
    let password = "contraseña123";
    let hashed = hash_password(password).unwrap();

    let is_valid = verify_password(&hashed, password).unwrap();
    assert!(is_valid);
}

#[test]
fn should_verify_password_with_spaces() {
    let password = "password with spaces";
    let hashed = hash_password(password).unwrap();

    let is_valid = verify_password(&hashed, password).unwrap();
    assert!(is_valid);
}

#[tokio::test]
async fn should_generate_access_and_refresh_tokens() {
    unsafe {
        std::env::set_var("JWT_SECRET", "test-secret-key-for-testing-only");
    }
    let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/auth_db")
           .expect("Failed to create pool");

    let user = User::new(
        "Test User".to_string(),
        "test@example.com".to_string(),
        "password123".to_string(),
    );

    let (access_token, refresh_token) = generate_tokens(&pool, &user).await.unwrap();

    assert_ne!(access_token, refresh_token);

    assert_eq!(access_token.matches('.').count(), 2);
    assert_eq!(refresh_token.matches('.').count(), 2);

    assert!(!access_token.trim().is_empty(), "Access token is empty");
    assert!(!refresh_token.trim().is_empty(), "Refresh token is empty");
}

#[tokio::test]
async fn should_have_unique_jti_for_each_token() {
    unsafe {
        std::env::set_var("JWT_SECRET", "test-secret-key-for-testing-only");
    }

    let user = User::new(
        "Test User".to_string(),
        "test@example.com".to_string(),
        "password123".to_string(),
    );
    let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost/auth_db")
           .expect("Failed to create pool");

    let (access_token1, refresh_token1) = generate_tokens(&pool,&user).await.unwrap();
    let (access_token2, refresh_token2) = generate_tokens(&pool,&user).await.unwrap();

    let access_claims1 = decode_access_token(&access_token1).unwrap();
    let refresh_claims1 = decode_access_token(&refresh_token1).unwrap();
    let access_claims2 = decode_access_token(&access_token2).unwrap();
    let refresh_claims2 = decode_access_token(&refresh_token2).unwrap();

    assert_ne!(access_claims1.jti, refresh_claims1.jti);
    assert_ne!(access_claims1.jti, access_claims2.jti);
    assert_ne!(refresh_claims1.jti, refresh_claims2.jti);
}

#[test]
fn should_handle_login_struct_creation() {
    let login = Login {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    assert_eq!(login.email, "test@example.com");
    assert_eq!(login.password, "password123");
}

#[test]
fn should_handle_empty_login_credentials() {
    let login = Login {
        email: "".to_string(),
        password: "".to_string(),
    };

    assert_eq!(login.email, "");
    assert_eq!(login.password, "");
}

#[test]
fn should_handle_login_with_special_characters() {
    let login = Login {
        email: "test+tag@example.com".to_string(),
        password: "!@#$%^&*()".to_string(),
    };

    assert_eq!(login.email, "test+tag@example.com");
    assert_eq!(login.password, "!@#$%^&*()");
}

#[test]
fn should_handle_login_with_unicode() {
    let login = Login {
        email: "josé@españa.es".to_string(),
        password: "contraseña123".to_string(),
    };

    assert_eq!(login.email, "josé@españa.es");
    assert_eq!(login.password, "contraseña123");
}

#[test]
fn should_handle_login_with_spaces() {
    let login = Login {
        email: "  test@example.com  ".to_string(),
        password: "  password 123  ".to_string(),
    };

    assert_eq!(login.email, "  test@example.com  ");
    assert_eq!(login.password, "  password 123  ");
}

#[test]
fn should_handle_very_long_password() {
    let long_password = "A".repeat(1000);
    let hashed = hash_password(&long_password).unwrap();

    let is_valid = verify_password(&hashed, &long_password).unwrap();
    assert!(is_valid);
}

#[test]
fn should_handle_very_short_password() {
    let short_password = "a";
    let hashed = hash_password(short_password).unwrap();

    let is_valid = verify_password(&hashed, short_password).unwrap();
    assert!(is_valid);
}

#[test]
fn should_handle_password_with_newlines() {
    let password_with_newlines = "password\nwith\nnewlines";
    let hashed = hash_password(password_with_newlines).unwrap();

    let is_valid = verify_password(&hashed, password_with_newlines).unwrap();
    assert!(is_valid);
}

#[test]
fn should_handle_password_with_tabs() {
    let password_with_tabs = "password\twith\ttabs";
    let hashed = hash_password(password_with_tabs).unwrap();

    let is_valid = verify_password(&hashed, password_with_tabs).unwrap();
    assert!(is_valid);
}

#[test]
fn should_handle_password_with_null_bytes() {
    let password_with_nulls = "password\0with\0nulls";
    let hashed = hash_password(password_with_nulls).unwrap();

    let is_valid = verify_password(&hashed, password_with_nulls).unwrap();
    assert!(is_valid);
}

#[test]
fn should_handle_claims_creation() {
    let user_id = Uuid::new_v4();
    let email = "test@example.com".to_string();
    let jti = Uuid::new_v4().to_string();
    let iat = chrono::Utc::now().timestamp() as usize;
    let exp = iat + 900;

    let claims = Claims {
        sub: user_id,
        email: email.clone(),
        jti: jti.clone(),
        roles: vec![],
        iat,
        exp,
        token_type: TokenType::Access,
    };

    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.email, email);
    assert_eq!(claims.jti, jti);
    assert_eq!(claims.iat, iat);
    assert_eq!(claims.exp, exp);
    assert_eq!(claims.token_type, TokenType::Access);
}

#[test]
fn should_handle_refresh_token_type() {
    let claims = Claims {
        sub: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        jti: Uuid::new_v4().to_string(),
        roles: vec![],
        iat: chrono::Utc::now().timestamp() as usize,
        exp: (chrono::Utc::now().timestamp() as usize) + 604800,
        token_type: TokenType::Refresh,
    };

    assert_eq!(claims.token_type, TokenType::Refresh);
}

#[test]
fn should_compare_token_types() {
    let access_type = TokenType::Access;
    let refresh_type = TokenType::Refresh;

    assert_eq!(access_type, TokenType::Access);
    assert_eq!(refresh_type, TokenType::Refresh);
    assert_ne!(access_type, refresh_type);
}

#[test]
fn should_handle_multiple_password_hashes() {
    let password = "same_password";
    let hash1 = hash_password(password).unwrap();
    let hash2 = hash_password(password).unwrap();

    assert_ne!(hash1, hash2);

    assert!(verify_password(&hash1, password).unwrap());
    assert!(verify_password(&hash2, password).unwrap());
}

#[test]
fn should_handle_case_sensitive_passwords() {
    let password1 = "Password123";
    let password2 = "password123";

    let hash1 = hash_password(password1).unwrap();
    let hash2 = hash_password(password2).unwrap();

    assert_ne!(hash1, hash2);

    assert!(verify_password(&hash1, password1).unwrap());
    assert!(!verify_password(&hash1, password2).unwrap());
    assert!(verify_password(&hash2, password2).unwrap());
    assert!(!verify_password(&hash2, password1).unwrap());
}
