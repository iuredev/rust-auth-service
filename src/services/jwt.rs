use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    errors::my_error::MyError,
    models::{auth::Claims, user::User},
};

pub fn generate_tokens(user: &User) -> Result<(String, String), MyError> {
    let now = chrono::Utc::now().timestamp() as usize;

    let access_claim = Claims {
        sub: user.id,
        email: user.email.clone(),
        // role: user.role.clone(),
        iat: now,
        exp: now + 60 * 15,
    };

    let refresh_claim = Claims {
        sub: user.id,
        email: user.email.clone(),
        // role: user.role.clone(),
        iat: now,
        exp: now + 60 * 60 * 24 * 7,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

    let access_token = encode(
        &Header::default(),
        &access_claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|err| MyError::Validation(err.to_string()))?;

    let refresh_token = encode(
        &Header::default(),
        &refresh_claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|err| MyError::Validation(err.to_string()))?;

    Ok((access_token, refresh_token))
}
