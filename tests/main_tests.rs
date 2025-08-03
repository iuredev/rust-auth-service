mod helpers;

#[cfg(test)]
mod tests {
    use super::helpers::*;

    #[test]
    fn test_create_test_user_data() {
        let user_data = create_test_user_data();

        assert!(user_data["name"].as_str().is_some());
        assert!(user_data["email"].as_str().is_some());
        assert!(user_data["password"].as_str().is_some());

        let email = user_data["email"].as_str().unwrap();
        assert!(email.contains("@example.com"));
    }

    #[test]
    fn test_create_login_data() {
        let email = "test@example.com";
        let login_data = create_test_login_data(email);

        assert_eq!(login_data["email"].as_str().unwrap(), email);
        assert_eq!(login_data["password"].as_str().unwrap(), "TestPassword123!");
    }

    #[test]
    fn test_generate_unique_email() {
        let email1 = generate_unique_email();
        let email2 = generate_unique_email();

        assert!(email1.contains("@example.com"));
        assert!(email2.contains("@example.com"));
        assert_ne!(email1, email2);
    }

    #[test]
    fn test_is_valid_uuid() {
        let valid_uuid = "123e4567-e89b-12d3-a456-426614174000";
        let invalid_uuid = "not-a-uuid";

        assert!(is_valid_uuid(valid_uuid));
        assert!(!is_valid_uuid(invalid_uuid));
    }

    #[test]
    fn test_is_valid_jwt_format() {
        let valid_jwt =
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let invalid_jwt = "not-a-jwt";

        assert!(is_valid_jwt_format(valid_jwt));
        assert!(!is_valid_jwt_format(invalid_jwt));
    }

    #[test]
    fn test_extract_token_from_response() {
        let response_with_token =
            r#"{"access_token": "test.token.here", "refresh_token": "refresh.token"}"#;
        let response_without_token = r#"{"message": "success"}"#;

        assert_eq!(
            extract_token_from_response(response_with_token),
            Some("test.token.here".to_string())
        );
        assert_eq!(extract_token_from_response(response_without_token), None);
    }

    #[test]
    fn test_assert_error_response() {
        let error_response = r#"{"error": "User not found"}"#;
        assert_error_response(error_response, "User not found");
    }

    #[test]
    fn test_assert_success_response() {
        let success_response = r#"{"message": "User created successfully"}"#;
        assert_success_response(success_response);
    }
}
