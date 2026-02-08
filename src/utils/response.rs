use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub meta: Meta,
    pub results: T,
}

#[derive(Serialize)]
pub struct Meta {
    pub status: String,
    pub message: String,
}

pub fn success_response<T: Serialize>(data: T, message: impl Into<String>) -> impl IntoResponse {
    Json(ApiResponse {
        meta: Meta {
            status: "success".to_string(),
            message: message.into(),
        },
        results: data,
    })
}
