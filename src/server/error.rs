use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RailboardApiError {
    pub domain: ErrorDomain,
    pub message: String,
    pub error: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorDomain {
    #[serde(rename = "vendo")]
    Vendo,
    #[serde(rename = "request")]
    Request,
}

pub type RailboardResult<T> = std::result::Result<T, RailboardApiError>;

impl IntoResponse for RailboardApiError {
    fn into_response(self) -> axum::response::Response {
        let code = match self.domain {
            ErrorDomain::Vendo => StatusCode::BAD_REQUEST,
            ErrorDomain::Request => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, Json(self)).into_response()
    }
}
