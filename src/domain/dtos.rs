use serde::{Deserialize, Serialize};
use crate::domain::entities::user::{Role, UserStatus};

/// Response DTO for user data (without password)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub name: String,
    pub phone: Option<String>,
    pub email: String,
    pub role: Role,
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
