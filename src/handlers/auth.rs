use axum::{extract::State, response::IntoResponse, Json};
use validator::Validate;
use crate::infrastructure::errors::AppError;
use crate::domain::dtos::RegisterUserDto;
use crate::usecases::auth::{RegisterUseCase, LoginUseCase, RefreshTokenUseCase};
use crate::utils::{response::success_response, validation::validate_request};
use crate::AppState;

#[derive(serde::Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 10, message = "Phone must be at least 10 characters"))]
    pub phone: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(serde::Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_request(&payload)?;

    let dto = RegisterUserDto {
        name: payload.name,
        phone: payload.phone,
        email: payload.email,
        password: payload.password,
    };

    let usecase = RegisterUseCase::new(state.user_repository.clone());
    let user = usecase.execute(dto).await?;

    Ok(success_response(user, "User registered successfully"))
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_request(&payload)?;

    let usecase = LoginUseCase::new(state.user_repository.clone(), state.jwt_service.clone());
    let tokens = usecase.execute(&payload.email, &payload.password).await?;

    Ok(success_response(tokens, "success"))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = RefreshTokenUseCase::new(state.user_repository.clone(), state.jwt_service.clone());
    let tokens = usecase.execute(&payload.refresh_token).await?;

    Ok(success_response(tokens, "success"))
}
