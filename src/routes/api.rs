use axum::{
    routing::{post, get, put, patch},
    Router,
};
use crate::handlers::auth::{sign_up, sign_in, refresh};
use crate::handlers::users::get_users;
use crate::handlers::user_management::{create_user, update_user, delete_user, update_user_status};
use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/auth/sign-up", post(sign_up))
        .route("/auth/sign-in", post(sign_in))
        .route("/auth/refresh", post(refresh))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", put(update_user).delete(delete_user))
        .route("/users/:id/status", patch(update_user_status))
}
