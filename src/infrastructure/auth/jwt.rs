use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::infrastructure::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String, // "access" or "refresh"
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_tokens(&self, user_id: Uuid) -> Result<(String, String), AppError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;

        // Access Token (15 minutes)
        let exp_access = (now + Duration::minutes(15)).timestamp() as usize;
        let access_claims = Claims {
            sub: user_id,
            exp: exp_access,
            iat,
            token_type: "access".to_string(),
        };
        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|_| AppError::TokenCreationError)?;

        // Refresh Token (7 days)
        let exp_refresh = (now + Duration::days(7)).timestamp() as usize;
        let refresh_claims = Claims {
            sub: user_id,
            exp: exp_refresh,
            iat,
            token_type: "refresh".to_string(),
        };
        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|_| AppError::TokenCreationError)?;

        Ok((access_token, refresh_token))
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenData<Claims>, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        ).map_err(|e| {
            tracing::error!("JWT Validation Error: {:?}", e);
            AppError::InvalidToken
        })
    }
}
