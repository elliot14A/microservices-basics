use std::fmt::{Display, self};
use axum::{http, response::IntoResponse};
use hyper::Response;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize, thiserror::Error)]
pub struct ApiErrResp {
    #[serde(serialize_with="serialize_statuscode")]
    code: http::StatusCode,
    message: String,
    error: String,
}

fn serialize_statuscode<S: Serializer>(code: &http::StatusCode, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u16(code.as_u16())
}

impl Display for ApiErrResp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]({}): {}", self.code, self.error, self.message)
    }
}

impl From<sqlx::Error> for ApiErrResp {
    fn from(_value: sqlx::Error) -> Self {
        todo!()
    }
}

impl IntoResponse for ApiErrResp {
    fn into_response(self) -> axum::response::Response {
        let payload = serde_json::to_string(&self).unwrap();
        let body = axum::body::boxed(axum::body::Full::from(payload));

        Response::builder().status(self.code).body(body).unwrap()
    }
}