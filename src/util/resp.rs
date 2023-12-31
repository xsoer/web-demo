use serde::Serialize;
use axum::response::{Response, IntoResponse};
use axum::http::StatusCode;
use axum::Json;

use crate::util::error::LibError;



#[derive(Serialize)]
pub struct Resp200<T>
    where
        T: Serialize,
{
    code: i32,
    msg: &'static str,
    data: T,
}

impl<T> Resp200<T>
    where
        T: Serialize,
{
    pub fn new(d: T) -> impl IntoResponse {
        Json(Resp200 {
            code: 200,
            msg: "success",
            data: d,
        })
    }
}


#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
    pub data: String,
}

impl LibError {
    fn body_code(&self) -> i32 {
        match self {
            _ => 40000,
        }
    }
    fn status_code(&self) -> StatusCode {
        match self {
            LibError::ParamsErr(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
            // LibError::BadEnv(_)
            // | LibError::SqlError(_)
            // | LibError::FormatError(_)
            // | LibError::ParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,

        }
    }
}

impl IntoResponse for LibError {
    fn into_response(self) -> Response {
        let err_rsp = ErrorResponse {
            code: self.body_code(),
            msg: self.to_string(),
            data: "".to_string(),
        };
        (self.status_code(), Json(err_rsp)).into_response()
    }
}