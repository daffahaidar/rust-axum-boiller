use crate::domain::dtos::{RegisterUserDto, AuthResponseDto, UserResponseDto};
use crate::infrastructure::errors::AppError;
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::entities::user::{User, Role};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::auth::password::{hash_password, verify_password};
use crate::infrastructure::auth::github::GitHubOAuthClient;
use crate::domain::dtos::GitHubUserInfo;

const ACCESS_TOKEN_EXPIRY_SECONDS: usize = 900; // 15 minutes

// Register Use Case
pub struct RegisterUseCase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> RegisterUseCase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, dto: RegisterUserDto) -> Result<UserResponseDto, AppError> {
        let password_hash = hash_password(&dto.password)?;

        let user = User {
            id: Uuid::new_v4(),
            name: dto.name,
            phone: dto.phone,
            email: dto.email,
            password_hash: Some(password_hash),
            role: Role::User,
            status: crate::domain::entities::user::UserStatus::default(),
            github_id: None,
            avatar_url: None,
            created_at: None,
            updated_at: None,
        };

        let created_user = self.user_repository.create(&user).await?;

        Ok(UserResponseDto {
            name: created_user.name,
            phone: created_user.phone,
            email: created_user.email,
            role: created_user.role,
            avatar_url: created_user.avatar_url,
        })
    }
}

// Login Use Case
pub struct LoginUseCase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_service: Arc<JwtService>,
}

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(user_repository: Arc<R>, jwt_service: Arc<JwtService>) -> Self {
        Self { user_repository, jwt_service }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<AuthResponseDto, AppError> {
        let user = self.user_repository.find_by_email(email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // OAuth-only users cannot sign in with password
        let password_hash = user.password_hash.as_ref()
            .ok_or(AppError::InvalidCredentials)?;

        if !verify_password(password_hash, password)? {
            return Err(AppError::InvalidCredentials);
        }

        let (access_token, refresh_token) = self.jwt_service.generate_tokens(&user)?;

        Ok(AuthResponseDto {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: ACCESS_TOKEN_EXPIRY_SECONDS,
        })
    }
}

// Refresh Token Use Case
pub struct RefreshTokenUseCase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_service: Arc<JwtService>,
}

impl<R: UserRepository> RefreshTokenUseCase<R> {
    pub fn new(user_repository: Arc<R>, jwt_service: Arc<JwtService>) -> Self {
        Self { user_repository, jwt_service }
    }

    pub async fn execute(&self, refresh_token: &str) -> Result<AuthResponseDto, AppError> {
        let claims = self.jwt_service.verify_token(refresh_token)?;
        
        if claims.claims.token_type != "refresh" {
             return Err(AppError::InvalidToken);
        }

        let user = self.user_repository.find_by_id(claims.claims.sub)
            .await?
            .ok_or(AppError::UserNotFound)?;

        let (access_token, refresh_token) = self.jwt_service.generate_tokens(&user)?;

        Ok(AuthResponseDto {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: ACCESS_TOKEN_EXPIRY_SECONDS,
        })
    }
}

// GitHub OAuth Callback Use Case
pub struct GitHubCallbackUseCase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_service: Arc<JwtService>,
    github_client: Arc<GitHubOAuthClient>,
}

impl<R: UserRepository> GitHubCallbackUseCase<R> {
    pub fn new(
        user_repository: Arc<R>,
        jwt_service: Arc<JwtService>,
        github_client: Arc<GitHubOAuthClient>,
    ) -> Self {
        Self {
            user_repository,
            jwt_service,
            github_client,
        }
    }

    pub async fn execute(&self, code: &str) -> Result<AuthResponseDto, AppError> {
        // 1. Exchange code for access token
        let access_token = self.github_client.exchange_code(code).await?;

        // 2. Fetch GitHub user info
        let github_user: GitHubUserInfo = self.github_client.get_user_info(&access_token).await?;

        let email = github_user.email
            .ok_or_else(|| AppError::OAuthError("GitHub account has no email".to_string()))?;

        let name = github_user.name.unwrap_or(github_user.login);

        // 3. Check if user already exists by github_id
        let user = if let Some(existing_user) = self.user_repository.find_by_github_id(github_user.id).await? {
            existing_user
        } else {
            // Check if a user with this email already exists (link accounts)
            if let Some(mut existing_user) = self.user_repository.find_by_email(&email).await? {
                // Link GitHub to existing account
                existing_user.github_id = Some(github_user.id);
                existing_user.avatar_url = github_user.avatar_url;
                self.user_repository.update(existing_user.id, &existing_user).await?;
                // Re-fetch to get updated data
                self.user_repository.find_by_github_id(github_user.id).await?
                    .ok_or(AppError::InternalServerError)?
            } else {
                // Create new user
                let new_user = User {
                    id: Uuid::new_v4(),
                    name,
                    phone: None,
                    email,
                    password_hash: None,
                    role: Role::User,
                    status: crate::domain::entities::user::UserStatus::default(),
                    github_id: Some(github_user.id),
                    avatar_url: github_user.avatar_url,
                    created_at: None,
                    updated_at: None,
                };
                self.user_repository.upsert_github_user(&new_user).await?
            }
        };

        // 4. Generate JWT tokens
        let (jwt_access_token, jwt_refresh_token) = self.jwt_service.generate_tokens(&user)?;

        Ok(AuthResponseDto {
            access_token: jwt_access_token,
            refresh_token: jwt_refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: ACCESS_TOKEN_EXPIRY_SECONDS,
        })
    }
}
