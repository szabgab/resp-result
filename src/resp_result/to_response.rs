use actix_web::HttpResponse;
use axum::{
    body::{boxed, Full},
    response::IntoResponse,
};
use http::{Response, StatusCode};

use crate::resp_error::RespError;

use super::RespResult;

fn prepare_respond<T, E>(r: RespResult<T, E>) -> Result<(Vec<u8>, StatusCode), serde_json::Error>
where
    T: serde::Serialize,
    E: RespError,
{
    let vec = serde_json::to_vec(&r);
    let body = match vec {
        Ok(body) => body,
        Err(e) => {
            #[cfg(feature = "log")]
            logger::error!("RespResult 响应出现异常 : {}", e);
            Err(e)?;
            unreachable!()
        }
    };

    let status = match r {
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
    Ok((body, status))
}

static JSON_TYPE: &str = "application/json";

#[cfg(feature = "for-axum")]
impl<T, E> IntoResponse for RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
{
    fn into_response(self) -> axum::response::Response {
        match prepare_respond(self) {
            Ok((body, status)) => Response::builder()
                .status(status)
                .header(http::header::CONTENT_TYPE, JSON_TYPE)
                .body(boxed(Full::from(body))),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(boxed(Full::from(e.to_string()))),
        }
        .expect("RespResult 构造响应时发生异常")
    }
}

impl<T, E> actix_web::Responder for RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match prepare_respond(self) {
            Ok((body, status)) => HttpResponse::build(status)
                .content_type(JSON_TYPE)
                .body(body),
            Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e.to_string()),
        }
    }
}
