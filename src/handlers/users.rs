use axum::{extract::State, response::IntoResponse};
use crate::AppState;
use crate::infrastructure::errors::AppError;
use crate::infrastructure::auth::middleware::AuthUser;
use crate::usecases::users::GetUsersUseCase;
use crate::domain::repositories::user_repository::UserRepository;
use crate::utils::response::success_response;

/// Handler for GET /users - restricted to Admin and SuperAdmin only
pub async fn get_users(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user_repository = state.user_repository.clone();
    let requester_id = auth_user.claims.claims.sub;
    
    let requester = user_repository
        .find_by_id(requester_id)
        .await?
        .ok_or(AppError::UserNotFound)?;
        
    let usecase = GetUsersUseCase::new(user_repository);
    let users = usecase.execute(requester.role).await?;

    Ok(success_response(users, "success"))
}
