use axum::{
    body::{boxed, Full},
    response::IntoResponse,
};
use http::{Response, StatusCode};

use crate::resp_error::RespError;

use super::RespResult;

impl<T: serde::Serialize, E: RespError> IntoResponse for RespResult<T, E> {
    fn into_response(self) -> axum::response::Response {
        let vec = serde_json::to_vec(&self);
        let body = match vec {
            Ok(body) => body,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Full::from(e.to_string())))
                    .unwrap();
            }
        };

        let status = match self {
            RespResult::Success(_) => StatusCode::OK,
            RespResult::Err(ref e) => e.http_code(),
        };

        Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(boxed(Full::from(body)))
            .unwrap()
    }
}
