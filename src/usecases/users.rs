use std::sync::Arc;
use crate::domain::entities::user::Role;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::dtos::UserResponseDto;
use crate::infrastructure::errors::AppError;

pub struct GetUsersUseCase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> GetUsersUseCase<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, role: Role) -> Result<Vec<UserResponseDto>, AppError> {
        // Authorize: Only Admin and SuperAdmin can get all users
        match role {
            Role::Admin | Role::SuperAdmin => {},
            _ => return Err(AppError::InvalidToken), // Or create a Forbidden error
        }

        let users = self.user_repository.find_all().await?;

        let user_dtos = users.into_iter().map(|u| UserResponseDto {
            name: u.name,
            phone: u.phone,
            email: u.email,
            role: u.role,
            avatar_url: u.avatar_url,
        }).collect();

        Ok(user_dtos)
    }
}
