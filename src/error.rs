use std::env::VarError;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response}
};
use thiserror::Error;
use serde_json::json;

#[derive(Error, Debug)]
pub enum UniAxNftErr {
    #[error("Config error: {0}")]
    ConfigErr(String),
    #[error("Database error: {0}")]
    DatabaseErr(String),
    #[error("Pinata error: {0}")]
    PinataErr(String),
    #[error("Solana error: {0}")]
    SolanaErr(String),
}

impl From<VarError> for UniAxNftErr {
    fn from(err: VarError) -> Self {
        UniAxNftErr::ConfigErr(err.to_string())
    }
}

impl IntoResponse for UniAxNftErr {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            UniAxNftErr::ConfigErr(err) => (StatusCode::SERVICE_UNAVAILABLE, err),
            UniAxNftErr::DatabaseErr(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            UniAxNftErr::PinataErr(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            UniAxNftErr::SolanaErr(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };

        let body = Json(json!({
            "error": err_msg,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type UniAxNftResult<T> = Result<T, UniAxNftErr>;
