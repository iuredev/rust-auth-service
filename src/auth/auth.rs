use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use redis::{AsyncCommands, aio::ConnectionManager};

use crate::{
    errors::my_error::MyError,
    models::{
        auth::{Claims, TokenType},
        user::User,
    },
};

pub fn generate_tokens(user: &User) -> Result<(String, String), MyError> {
    let now = chrono::Utc::now().timestamp() as usize;

    let access_claim = Claims {
        sub: user.id,
        email: user.email.clone(),
        // role: user.role.clone(),
        jti: uuid::Uuid::new_v4().to_string(),
        iat: now,
        exp: now + 60 * 15,
        token_type: TokenType::Access,
    };

    let refresh_claim = Claims {
        sub: user.id,
        email: user.email.clone(),
        // role: user.role.clone(),
        jti: uuid::Uuid::new_v4().to_string(),
        iat: now,
        exp: now + 60 * 60 * 24 * 7,
        token_type: TokenType::Refresh,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

    let access_token = encode(
        &Header::new(Algorithm::HS256),
        &access_claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|err| MyError::Validation(err.to_string()))?;

    let refresh_token = encode(
        &Header::new(Algorithm::HS256),
        &refresh_claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|err| MyError::Validation(err.to_string()))?;

    Ok((access_token, refresh_token))
}

pub fn decode_access_token(token: &str) -> Result<Claims, MyError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
    let data = decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| MyError::Validation("The token has expired or is invalid".to_string()))?;

    Ok(data.claims)
}

pub async fn validate_jwt(
    redis: &mut ConnectionManager,
    access_token: &str,
) -> Result<Claims, MyError> {
    let claims = decode_access_token(access_token)?;

    let key_jti = format!("jti_revoked:{}", claims.jti);

    let is_revoked: Option<bool> = redis.get(key_jti).await.unwrap();

    if is_revoked.is_some() {
        return Err(MyError::Validation(
            "The token expired or is invalid".to_string(),
        ));
    }

    Ok(claims)
}
