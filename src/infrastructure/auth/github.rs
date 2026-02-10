use reqwest::Client;
use crate::domain::dtos::{GitHubTokenResponse, GitHubUserInfo, GitHubEmail};
use crate::infrastructure::errors::AppError;

pub struct GitHubOAuthClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: Client,
}

impl GitHubOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            http_client: Client::new(),
        }
    }

    /// Returns the GitHub authorization URL to redirect the user to
    pub fn get_authorize_url(&self) -> String {
        format!(
            "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email",
            self.client_id, self.redirect_uri
        )
    }

    /// Exchange the authorization code for an access token
    pub async fn exchange_code(&self, code: &str) -> Result<String, AppError> {
        let response = self
            .http_client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .json(&serde_json::json!({
                "client_id": self.client_id,
                "client_secret": self.client_secret,
                "code": code,
                "redirect_uri": self.redirect_uri,
            }))
            .send()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to exchange code: {}", e)))?;

        let token_response: GitHubTokenResponse = response
            .json()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to parse token response: {}", e)))?;

        Ok(token_response.access_token)
    }

    /// Fetch the authenticated GitHub user's profile
    pub async fn get_user_info(&self, access_token: &str) -> Result<GitHubUserInfo, AppError> {
        let mut user_info: GitHubUserInfo = self
            .http_client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "rust-axum-app")
            .send()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to fetch user info: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::OAuthError(format!("Failed to parse user info: {}", e)))?;

        // If email is not public, fetch from /user/emails endpoint
        if user_info.email.is_none() {
            let emails: Vec<GitHubEmail> = self
                .http_client
                .get("https://api.github.com/user/emails")
                .header("Authorization", format!("Bearer {}", access_token))
                .header("User-Agent", "rust-axum-app")
                .send()
                .await
                .map_err(|e| AppError::OAuthError(format!("Failed to fetch emails: {}", e)))?
                .json()
                .await
                .map_err(|e| AppError::OAuthError(format!("Failed to parse emails: {}", e)))?;

            // Find the primary verified email
            user_info.email = emails
                .into_iter()
                .find(|e| e.primary && e.verified)
                .map(|e| e.email);
        }

        Ok(user_info)
    }
}
