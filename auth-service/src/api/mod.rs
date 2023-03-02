use axum::{response::IntoResponse, routing::get};

use crate::{error::ApiErrResp, context::RawAuthContext};
use axum::http;
mod register;


type Result<T> = std::result::Result<T, ApiErrResp>;

async fn health_handler() -> Result<impl IntoResponse> {
    Ok(http::StatusCode::OK)
}

pub fn routes<H: RawAuthContext>() -> axum::Router {
    axum::Router::new().route("/", get(health_handler))
}