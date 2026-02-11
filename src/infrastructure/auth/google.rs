use reqwest::Client;
use crate::domain::dtos::{GoogleTokenResponse, GoogleUserInfo};
use crate::infrastructure::errors::AppError;

pub struct GoogleOAuthClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: Client,
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            http_client: Client::new(),
        }
    }

    /// Returns the Google authorization URL to redirect the user to
    pub fn get_authorize_url(&self) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&access_type=offline",
            self.client_id, self.redirect_uri
        )
    }

    /// Exchange the authorization code for tokens
    pub async fn exchange_code(&self, code: &str) -> Result<GoogleTokenResponse, AppError> {
        let response = self
            .http_client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to exchange code: {}", e)))?;

        let token_response: GoogleTokenResponse = response
            .json()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to parse token response: {}", e)))?;

        Ok(token_response)
    }

    /// Fetch the authenticated Google user's profile
    pub async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo, AppError> {
        let user_info: GoogleUserInfo = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to fetch user info: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to parse user info: {}", e)))?;

        Ok(user_info)
    }
}
