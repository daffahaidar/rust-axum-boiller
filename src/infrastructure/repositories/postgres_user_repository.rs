use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::errors::AppError;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<User, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, phone, email, password_hash, role, status, github_id, avatar_url)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            "#,
        )
        .bind(&user.name)
        .bind(&user.phone)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.role)
        .bind(&user.status)
        .bind(&user.github_id)
        .bind(&user.avatar_url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            // Check for unique constraint violation
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().unwrap_or_default() == "23505" {
                    return AppError::EmailAlreadyExists;
                }
            }
            AppError::DatabaseError(e)
        })?;

        Ok(rec)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(rec)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(rec)
    }

    async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(rec)
    }

    async fn update(&self, id: Uuid, user: &User) -> Result<User, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET name = $1, phone = $2, email = $3, role = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            "#,
        )
        .bind(&user.name)
        .bind(&user.phone)
        .bind(&user.email)
        .bind(&user.role)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().unwrap_or_default() == "23505" {
                    return AppError::EmailAlreadyExists;
                }
            }
            AppError::DatabaseError(e)
        })?;

        Ok(rec)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    async fn update_status(&self, id: Uuid, status: crate::domain::entities::user::UserStatus) -> Result<User, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(rec)
    }

    async fn find_by_github_id(&self, github_id: i64) -> Result<Option<User>, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            FROM users
            WHERE github_id = $1
            "#,
        )
        .bind(github_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(rec)
    }

    async fn upsert_github_user(&self, user: &User) -> Result<User, AppError> {
        let rec = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, name, email, github_id, avatar_url, role, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (github_id) DO UPDATE
            SET name = EXCLUDED.name,
                avatar_url = EXCLUDED.avatar_url,
                updated_at = NOW()
            RETURNING id, name, phone, email, password_hash, role, status, github_id, avatar_url, created_at, updated_at
            "#,
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.github_id)
        .bind(&user.avatar_url)
        .bind(&user.role)
        .bind(&user.status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("GitHub user upsert error: {:?}", e);
            AppError::DatabaseError(e)
        })?;

        Ok(rec)
    }
}
