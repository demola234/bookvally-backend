use chrono::Utc;
use jsonwebtoken::{self};
use kernel::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_ttl_secs: i64,
    pub refresh_ttl_secs: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub handle: String,
    pub iat: i64,
    pub token_type: String,
}

impl JwtConfig {
    pub fn new(secret: String, access_ttl_secs: i64, refresh_ttl_secs: i64) -> Self {
        Self {
            secret,
            access_ttl_secs,
            refresh_ttl_secs,
        }
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub fn get_access_ttl_secs(&self) -> i64 {
        self.access_ttl_secs
    }

    pub fn get_refresh_ttl_secs(&self) -> i64 {
        self.refresh_ttl_secs
    }
}

pub fn encode_access(
    config: &JwtConfig,
    user_id: UserId,
    handle: String,
) -> anyhow::Result<String> {
    let now = Utc::now();
    let exp = now + chrono::Duration::seconds(config.get_access_ttl_secs());
    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp(),
        handle,
        iat: now.timestamp(),
        token_type: "access".to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(config.get_secret().as_bytes()),
    )?;

    Ok(token)
}

pub fn encode_refresh(
    config: &JwtConfig,
    user_id: UserId,
    handle: String,
) -> anyhow::Result<String> {
    let now = Utc::now();
    let exp = now + chrono::Duration::seconds(config.get_refresh_ttl_secs());
    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp(),
        handle,
        iat: now.timestamp(),
        token_type: "refresh".to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(config.get_secret().as_bytes()),
    )?;
    Ok(token)
}

pub fn decode_access(config: &JwtConfig, token: &str) -> anyhow::Result<Claims> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(config.get_secret().as_bytes()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token_data.claims)
}
