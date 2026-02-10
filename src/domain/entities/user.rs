use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "PascalCase")]
pub enum Role {
    Admin,
    SuperAdmin,
    User,
    Mentor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub enum UserStatus {
    Active,
    Suspended,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub role: Role,
    pub status: UserStatus,
    pub github_id: Option<i64>,
    pub avatar_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
