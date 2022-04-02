use std::convert::Infallible;
#[allow(unused_imports)]
use std::str::FromStr;

use http::{header::HeaderName, HeaderValue, StatusCode};

use crate::{get_config, resp_error::RespError};

use super::RespResult;
#[allow(dead_code)]
#[inline]
fn prepare_respond<T, E>(
    r: &RespResult<T, E>,
) -> Result<(Vec<u8>, StatusCode, Option<(HeaderName, HeaderValue)>), serde_json::Error>
where
    T: serde::Serialize,
    E: RespError,
{
    #[allow(unused_variables)]
    let cfg = &get_config().resp;

    let vec = serde_json::to_vec(r);
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
            logger::debug!("RespResult 接管的 成功 响应",);
            StatusCode::OK
        }
        RespResult::Err(ref e) => {
            #[cfg(feature = "log")]
            logger::debug!(
                "RespResult 接管的 异常 响应 | {} => {}",
                std::any::type_name::<E>(),
                e.description()
            );
            e.http_code()
        }
    };
    #[cfg(feature = "extra-code")]
    let r_header = {
        match r {
            RespResult::Success(_) => None,
            RespResult::Err(e) => {
                if let Some(n) = cfg.extra_code {
                    Some((
                        HeaderName::from_str(n).expect("Bad HeaderName"),
                        HeaderValue::from_str(&e.extra_code().to_string())
                            .expect("Bad HeaderValue"),
                    ))
                } else {
                    None
                }
            }
        }
    };
    #[cfg(not(feature = "extra-code"))]
    let r_header = None;

    #[cfg(feature = "log")]
    logger::info!(
        "RespResult 响应 准备构造 Status :{} BodySize: {}",
        status,
        body.len()
    );
    Ok((body, status, r_header))
}
#[cfg(feature = "mime")]
#[allow(dead_code)]
static JSON_TYPE: &'static mime::Mime = &mime::APPLICATION_JSON;

#[cfg(feature = "for-axum")]
impl<T, E> axum::response::IntoResponse for RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
{
    #[inline]
    fn into_response(self) -> axum::response::Response {
        match prepare_respond(&self) {
            Ok((body, status, eh)) => match eh {
                None => axum::response::Response::builder()
                    .status(status)
                    .header(http::header::CONTENT_TYPE, JSON_TYPE.as_ref())
                    .body(axum::body::boxed(axum::body::Full::from(body))),
                Some((k, v)) => axum::response::Response::builder()
                    .status(status)
                    .header(http::header::CONTENT_TYPE, JSON_TYPE.as_ref())
                    .header(k, v)
                    .body(axum::body::boxed(axum::body::Full::from(body))),
            },
            Err(e) => axum::response::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::boxed(axum::body::Full::from(e.to_string()))),
        }
        .expect("RespResult 构造响应时发生异常")
    }
}

#[cfg(feature = "for-actix")]
impl<T, E> actix_web::Responder for RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
{
    type Body = actix_web::body::BoxBody;

    #[inline]
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        to_actix_resp(&self)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Nil;

impl From<()> for Nil {
    fn from(_: ()) -> Self {
        Self
    }
}

impl From<Infallible> for Nil {
    fn from(_: Infallible) -> Self {
        Self
    }
}

impl std::fmt::Display for Nil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nil")
    }
}

#[cfg(feature = "for-actix")]
impl<E> actix_web::ResponseError for RespResult<Nil, E>
where
    E: RespError,
{
    fn status_code(&self) -> StatusCode {
        match self {
            RespResult::Err(e) => e.http_code(),
            RespResult::Success(_) => StatusCode::OK,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        to_actix_resp(self)
    }
}

#[cfg(feature = "for-actix")]
fn to_actix_resp<T, E>(this: &RespResult<T, E>) -> actix_web::HttpResponse
where
    T: serde::Serialize,
    E: RespError,
{
    match prepare_respond(this) {
        Ok((body, status, Some(e_header))) => actix_web::HttpResponse::build(status)
            .content_type(JSON_TYPE.as_ref())
            .insert_header(e_header)
            .body(body),
        Ok((body, status, None)) => actix_web::HttpResponse::build(status)
            .content_type(JSON_TYPE.as_ref())
            .body(body),
        Err(err) => {
            let body = format!(r#"{{"panic-error":"序列化响应体失败","err-msg":{}}}"#, err);
            actix_web::HttpResponse::InternalServerError()
                .content_type(JSON_TYPE.as_ref())
                .body(body)
        }
    }
}
