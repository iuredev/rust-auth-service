use rust_auth_service::utils::password::{hash_password, verify_password};

#[test]
fn test_hash_password() {
    let password = "password";
    let hash = hash_password(password).unwrap();
    assert!(verify_password(&hash, password).unwrap());
}
