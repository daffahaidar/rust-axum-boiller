use axum::{
    extract::{FromRequestParts, FromRef},
    http::request::Parts,
    RequestPartsExt,
};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::TokenData;
use crate::AppState;
use crate::infrastructure::auth::jwt::Claims;
use crate::infrastructure::errors::AppError;

pub struct AuthUser {
    pub claims: TokenData<Claims>,
}

// Manual implementation to avoid lifetime issues with async_trait
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        // Extract the token from the Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| {
                tracing::error!("Auth Header Extraction Error: {:?}", e);
                AppError::InvalidToken
            })?;

        // Verify the token
        let token_data = state.jwt_service.verify_token(bearer.token())?;
        
        // Ensure it is an access token
        if token_data.claims.token_type != "access" {
            return Err(AppError::InvalidToken);
        }

        Ok(AuthUser { claims: token_data })
    }
}
