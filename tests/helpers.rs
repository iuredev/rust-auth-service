use serde_json::json;
use uuid::Uuid;

pub fn create_test_user_data() -> serde_json::Value {
    json!({
        "name": "Test User",
        "email": format!("test_{}@example.com", Uuid::new_v4()),
        "password": "TestPassword123!"
    })
}

pub fn create_test_login_data(email: &str) -> serde_json::Value {
    json!({
        "email": email,
        "password": "TestPassword123!"
    })
}

pub fn assert_error_response(response_body: &str, expected_error: &str) {
    let json: serde_json::Value = serde_json
        ::from_str(response_body)
        .expect("Failed to parse response as JSON");

    assert!(json.get("error").is_some(), "Response should contain error field");
    assert_eq!(json["error"].as_str().unwrap(), expected_error, "Error message mismatch");
}

pub fn assert_success_response(response_body: &str) {
    let json: serde_json::Value = serde_json
        ::from_str(response_body)
        .expect("Failed to parse response as JSON");

    assert!(json.get("error").is_none(), "Response should not contain error field");
}

pub fn generate_unique_email() -> String {
    format!("test_{}@example.com", Uuid::new_v4())
}

pub fn is_valid_uuid(uuid_str: &str) -> bool {
    uuid_str.parse::<Uuid>().is_ok()
}

pub fn is_valid_jwt_format(token: &str) -> bool {
    token.split('.').count() == 3
}

pub fn extract_token_from_response(response_body: &str) -> Option<String> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(response_body) {
        json.get("access_token")
            .and_then(|token| token.as_str())
            .map(|s| s.to_string())
    } else {
        None
    }
}
