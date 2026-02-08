use crate::domain::dtos::{RegisterUserDto, AuthResponseDto, UserResponseDto};
use crate::infrastructure::errors::AppError;
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::entities::user::{User, Role};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::auth::password::{hash_password, verify_password};

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
            password_hash,
            role: Role::User,
            status: crate::domain::entities::user::UserStatus::default(),
            created_at: None,
            updated_at: None,
        };

        let created_user = self.user_repository.create(&user).await?;

        Ok(UserResponseDto {
            name: created_user.name,
            phone: created_user.phone,
            email: created_user.email,
            role: created_user.role,
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

        if !verify_password(&user.password_hash, password)? {
            return Err(AppError::InvalidCredentials);
        }

        let (access_token, refresh_token) = self.jwt_service.generate_tokens(user.id)?;

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

        let (access_token, refresh_token) = self.jwt_service.generate_tokens(user.id)?;

        Ok(AuthResponseDto {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: ACCESS_TOKEN_EXPIRY_SECONDS,
        })
    }
}
