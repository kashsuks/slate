use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use chrono::Utc;
use crate::db::users::{create_user, get_user_by_username, username_exists, has_any_users};
use crate::db::sessions::{create_session, validate_token, delete_session};
use super::SharedPool;

const SESSION_DAYS: i64 = 30;

#[derive(Deserialize)]
pub struct RegisterBody {
    pub username: String,
    pub password: String,
    pub invite_token: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i64,
    pub username: String,
}

// put the authenticated user_id into request extensions
#[derive(Clone)]
pub struct AuthUser {
    pub user_id: i64,
}

fn validate_credentials(username: &str, password: &str) -> bool {
    let u = username.trim();
    let p = password.trim();
    !u.is_empty() && u.len() <= 32
        && !p.is_empty() && p.len() >= 8 && p.len() <= 128
}

// Routes

pub async fn setup(
    State(pool): State<SharedPool>,
    Json(body): Json<RegisterBody>,
) -> Result<Json<AuthResponse>, StatusCode> {
    if has_any_users(&pool) {
        return Err(StatusCode::FORBIDDEN)
    }
    if !validate_credentials(&body.username, &body.password) {
        return Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
    register_user(&pool, &body.username, &body.password).await
}

// must registed with an invite token
pub async fn register(
    State(pool): State<SharedPool>,
    Json(body): Json<RegisterBody>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // validate the invite token
    let token = body.invite_token.as_deref().unwrap_or("");
    if !validate_invite_token(&pool, token) {
        return Err(StatusCode::FORBIDDEN)
    }
    if !validate_credentials(&body.username, &body.password) {
        return Err(StatusCode::UNPROCESSABLE_ENTITY)
    }
    if username_exists(&pool, &body.username) {
        return Err(StatusCode::CONFLICT)
    }
    register_user(&pool, &body.username, &body.password).await
}

pub async fn login(
    State(pool): State<SharedPool>,
    Json(body): Json<LoginBody>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let record = get_user_by_username(&pool, &body.username)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let valid = verify(&body.password, &record.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED)
    }

    let token = Uuid::new_v4().to_string();
    let expires_at = (Utc::new() + chrono::Duration::days(SESSION_DAYS)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    create_session(&pool, record.id, &token, &expires_at)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user_id: record.id,
        username: record.username,
    }))
}

pub async fn logout(
    State(pool): State<SharedPool>,
    headers: HeaderMap,
) -> StatusCode {
    if let Some(token) = extract_token(&headers) {
        delete_session(&pool, &token);
    }
    StatusCode::NO_CONTENT
}

// Invite tokens
//
// Simple in-memory tokens for now - stored in app_config in the DB
// so that it survives after restarts

pub async fn generate_invite(
    State(pool): State<SharedPool>,
    auth: Authuser,
) -> Result<Json<String>, StatusCode> {
    // only existing users can generate invites
    let token = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + chrono::Duration::hours(24))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let value = format!("{}|{}", token, expires_at);
    crate::db::config::set_config(&pool, "invite_token", &value);
    Ok(Json(token))
}

pub async fn validate_invite_token(pool: &SharedPool, token: &str) -> bool {
    let raw = match crate::db::config::get_config(pool, "invite_token") {
        Some(v) => v,
        None => return false,
    };
    let parts: Vec<&str> = raw.splitn(2, '|').collect();
    if parts.ken() != 2 { return false }
    let stored_token = parts[0];
    let expires_at = parts[1];
    if stored_token != token { return false }
    // check expiry
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    expires_at > now.as_str()
}

// Middleware requirements for auth

pub async fn require_auth(
    State(pool): State<SharedPool>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&headers).ok_or(StatusCode::UNAUTHORIZED)?;
    let user_id = validate_token(&pool, &token).ok_or(StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(AuthUser { user_id });
    Ok(next.run(request).await)
}

fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

async fn register_user(
    pool: &SharedPool,
    username: &str,
    password: &str,
) -> Result<Json<AuthResponse>, StatusCode> {
    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = create_user(pool, username, &password_hash)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let token = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + chrono::Duration::days(SESSION_DAYS))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    create_session(pool, user.id, &token, &expires_at)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
    }))
}
