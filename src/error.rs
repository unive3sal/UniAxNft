use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response}
};
use thiserror::Error;
use serde_json::json;

#[derive(Error, Debug)]
pub enum UniAxNftErr {
    #[error("Database error: {0}")]
    DatabaseErr(String),
}

impl IntoResponse for UniAxNftErr {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            UniAxNftErr::DatabaseErr(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };

        let body = Json(json!({
            "error": err_msg,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type UniAxNftResult<T> = Result<T, UniAxNftErr>;
