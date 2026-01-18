use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::error::{UniAxNftErr, UniAxNftResult};
use crate::middleware::auth::{Authorizer, UserInfo};
use crate::state::UniAxNftState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

pub async fn user_register(
    State(state): State<UniAxNftState>,
    Json(payload): Json<RegisterRequest>,
) -> UniAxNftResult<impl IntoResponse> {
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| UniAxNftErr::AuthErr(format!("Failed to hash password: {}", e)))?;

    let row = sqlx::query(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, username
        "#,
    )
    .bind(&payload.email)
    .bind(&payload.username)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            UniAxNftErr::AuthErr("Email or username already exists".to_string())
        }
        _ => UniAxNftErr::DatabaseErr(format!("Failed to create user: {}", e)),
    })?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
        }),
    ))
}

pub async fn user_login(
    State(state): State<UniAxNftState>,
    Json(payload): Json<LoginRequest>,
) -> UniAxNftResult<impl IntoResponse> {
    let row = sqlx::query(
        r#"
        SELECT id, email, username, password_hash
        FROM users
        WHERE email = $1 AND is_active = true
        "#,
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| UniAxNftErr::DatabaseErr(format!("Database error: {}", e)))?
    .ok_or_else(|| UniAxNftErr::AuthErr("Invalid email or password".to_string()))?;

    let user_id: Uuid = row.get("id");
    let email: String = row.get("email");
    let username: String = row.get("username");
    let password_hash: String = row.get("password_hash");

    let password_valid = bcrypt::verify(&payload.password, &password_hash)
        .map_err(|e| UniAxNftErr::AuthErr(format!("Password verification failed: {}", e)))?;

    if !password_valid {
        return Err(UniAxNftErr::AuthErr(
            "Invalid email or password".to_string(),
        ));
    }

    // Update last_login_at
    sqlx::query(r#"UPDATE users SET last_login_at = CURRENT_TIMESTAMP WHERE id = $1"#)
        .bind(user_id)
        .execute(&state.db)
        .await
        .map_err(|e| UniAxNftErr::DatabaseErr(format!("Failed to update last login: {}", e)))?;

    let token = Authorizer::generate_token(user_id, &email)?;

    Ok(Json(LoginResponse {
        token,
        user: UserResponse {
            id: user_id,
            email,
            username,
        },
    }))
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub async fn change_password(
    State(state): State<UniAxNftState>,
    Extension(user_info): Extension<UserInfo>,
    Json(payload): Json<ChangePasswordRequest>,
) -> UniAxNftResult<impl IntoResponse> {
    let row = sqlx::query(
        r#"
        SELECT password_hash
        FROM users
        WHERE id = $1 AND is_active = true
        "#,
    )
    .bind(user_info.user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| UniAxNftErr::DatabaseErr(format!("Database error: {}", e)))?
    .ok_or_else(|| UniAxNftErr::AuthErr("User not found".to_string()))?;

    let password_hash: String = row.get("password_hash");

    let password_valid = bcrypt::verify(&payload.current_password, &password_hash)
        .map_err(|e| UniAxNftErr::AuthErr(format!("Password verification failed: {}", e)))?;

    if !password_valid {
        return Err(UniAxNftErr::AuthErr(
            "Current password is incorrect".to_string(),
        ));
    }

    let new_password_hash = bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST)
        .map_err(|e| UniAxNftErr::AuthErr(format!("Failed to hash password: {}", e)))?;

    sqlx::query(
        r#"UPDATE users SET password_hash = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"#,
    )
    .bind(&new_password_hash)
    .bind(user_info.user_id)
    .execute(&state.db)
    .await
    .map_err(|e| UniAxNftErr::DatabaseErr(format!("Failed to update password: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
