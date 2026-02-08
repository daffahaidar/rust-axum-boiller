use axum::{extract::{State, Path}, response::IntoResponse};
use validator::Validate;
use uuid::Uuid;
use crate::AppState;
use crate::infrastructure::errors::AppError;
use crate::infrastructure::auth::middleware::AuthUser;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::dtos::{CreateUserDto, UpdateUserDto, UpdateUserStatusDto};
use crate::usecases::user_management::{
    CreateUserUseCase, UpdateUserUseCase, DeleteUserUseCase, UpdateUserStatusUseCase
};
use crate::utils::{response::success_response, validation::validate_request};

#[derive(serde::Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub phone: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub role: crate::domain::entities::user::Role,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub role: Option<crate::domain::entities::user::Role>,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserStatusRequest {
    pub status: crate::domain::entities::user::UserStatus,
}

/// POST /api/v1/users - Create user (Admin + SuperAdmin)
pub async fn create_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    axum::Json(payload): axum::Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_request(&payload)?;

    let requester_id = auth_user.claims.claims.sub;
    let requester = state.user_repository
        .find_by_id(requester_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    let dto = CreateUserDto {
        name: payload.name,
        phone: payload.phone,
        email: payload.email,
        password: payload.password,
        role: payload.role,
    };

    let usecase = CreateUserUseCase::new(state.user_repository.clone());
    let user = usecase.execute(requester.role, dto).await?;

    Ok(success_response(user, "User created successfully"))
}

/// PUT /api/v1/users/:id - Update user (SuperAdmin only)
pub async fn update_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
    axum::Json(payload): axum::Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let requester_id = auth_user.claims.claims.sub;
    let requester = state.user_repository
        .find_by_id(requester_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    let dto = UpdateUserDto {
        name: payload.name,
        phone: payload.phone,
        email: payload.email,
        role: payload.role,
    };

    let usecase = UpdateUserUseCase::new(state.user_repository.clone());
    let user = usecase.execute(requester.role, user_id, dto).await?;

    Ok(success_response(user, "User updated successfully"))
}

/// DELETE /api/v1/users/:id - Delete user (SuperAdmin only, not self)
pub async fn delete_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let requester_id = auth_user.claims.claims.sub;
    let requester = state.user_repository
        .find_by_id(requester_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    let usecase = DeleteUserUseCase::new(state.user_repository.clone());
    usecase.execute(requester_id, requester.role, user_id).await?;

    Ok(success_response((), "User deleted successfully"))
}

/// PATCH /api/v1/users/:id/status - Suspend/activate user (Admin + SuperAdmin)
pub async fn update_user_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
    axum::Json(payload): axum::Json<UpdateUserStatusRequest>,
) -> Result<impl IntoResponse, AppError> {
    let requester_id = auth_user.claims.claims.sub;
    let requester = state.user_repository
        .find_by_id(requester_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    let dto = UpdateUserStatusDto {
        status: payload.status,
    };

    let usecase = UpdateUserStatusUseCase::new(state.user_repository.clone());
    let user = usecase.execute(requester.role, user_id, dto).await?;

    Ok(success_response(user, "User status updated successfully"))
}
