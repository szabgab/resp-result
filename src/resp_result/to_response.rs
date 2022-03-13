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
                #[cfg(feature = "log")]
                logger::error!("RespResult 响应出现异常 : {}", e);

                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Full::from(e.to_string())))
                    .expect("RespResult 构造响应时发生异常")
            }
        };

        let status = match self {
            RespResult::Success(_) => {
                #[cfg(feature = "log")]
                logger::info!("RespResult 接管的 成功 响应",);
                StatusCode::OK
            }
            RespResult::Err(ref e) => {
                #[cfg(feature = "log")]
                logger::info!(
                    "RespResult 接管的 异常 响应 | {} => {}",
                    std::any::type_name::<E>(),
                    e.description()
                );
                e.http_code()
            }
        };

        #[cfg(feature = "log")]
        logger::info!(
            "RespResult 响应 准备构造 Status :{} BodySize: {}",
            status,
            body.len()
        );

        Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(boxed(Full::from(body)))
            .expect("RespResult 构造响应时发生异常")
    }
}
