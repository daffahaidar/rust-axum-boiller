use serde::{Deserialize, Serialize};
use crate::domain::entities::user::{Role, UserStatus};

/// Response DTO for user data (without password)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub name: String,
    pub phone: Option<String>,
    pub email: String,
    pub role: Role,
    pub avatar_url: Option<String>,
}

/// Request DTO for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterUserDto {
    pub name: String,
    pub phone: Option<String>,
    pub email: String,
    pub password: String,
}

/// Response DTO for authentication with tokens
#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

/// Request DTO for creating a user (Admin/SuperAdmin)
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub phone: Option<String>,
    pub email: String,
    pub password: String,
    pub role: Role,
}

/// Request DTO for updating a user (SuperAdmin only)
#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
}

/// Request DTO for updating user status
#[derive(Debug, Deserialize)]
pub struct UpdateUserStatusDto {
    pub status: UserStatus,
}

/// GitHub user info from GitHub API
#[derive(Debug, Deserialize)]
pub struct GitHubUserInfo {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

/// GitHub access token response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GitHubTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

/// GitHub email response (from /user/emails endpoint)
#[derive(Debug, Deserialize)]
pub struct GitHubEmail {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}

/// Google access token response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

/// Google user info from userinfo endpoint
#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub verified_email: Option<bool>,
}
