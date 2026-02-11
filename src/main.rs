mod config;
mod domain;
mod handlers;
mod infrastructure;
mod routes;
mod usecases;
mod utils;

use axum::http::Method;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::auth::github::GitHubOAuthClient;
use crate::infrastructure::auth::google::GoogleOAuthClient;
use crate::infrastructure::database::postgres::Database;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<PostgresUserRepository>,
    pub jwt_service: Arc<JwtService>,
    pub github_oauth: Arc<GitHubOAuthClient>,
    pub google_oauth: Arc<GoogleOAuthClient>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let github_client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let github_client_secret = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set");
    let github_redirect_uri = env::var("GITHUB_REDIRECT_URI").expect("GITHUB_REDIRECT_URI must be set");
    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let google_redirect_uri = env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");

    let db = Database::new(&database_url).await.expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&db.pool)
        .await
        .expect("Failed to run migrations");

    let user_repository = Arc::new(PostgresUserRepository::new(db.pool.clone()));
    let jwt_service = Arc::new(JwtService::new(jwt_secret));
    let github_oauth = Arc::new(GitHubOAuthClient::new(
        github_client_id,
        github_client_secret,
        github_redirect_uri,
    ));
    let google_oauth = Arc::new(GoogleOAuthClient::new(
        google_client_id,
        google_client_secret,
        google_redirect_uri,
    ));

    let state = AppState {
        user_repository,
        jwt_service,
        github_oauth,
        google_oauth,
    };

    let api_routes = routes::api::create_router();
    
    let app = axum::Router::new()
        .nest("/api/v1", api_routes)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Server listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
