use async_trait::async_trait;
use uuid::Uuid;
use super::super::entities::user::User;
use crate::infrastructure::errors::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_all(&self) -> Result<Vec<User>, AppError>;
    async fn update(&self, id: Uuid, user: &User) -> Result<User, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn update_status(&self, id: Uuid, status: crate::domain::entities::user::UserStatus) -> Result<User, AppError>;
    async fn find_by_github_id(&self, github_id: i64) -> Result<Option<User>, AppError>;
    async fn upsert_github_user(&self, user: &User) -> Result<User, AppError>;
    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, AppError>;
    async fn upsert_google_user(&self, user: &User) -> Result<User, AppError>;
}
