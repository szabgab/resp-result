mod actix;
pub mod axum;

#[allow(unused_imports)]
use std::str::FromStr;

use http::{header::HeaderName, HeaderValue, StatusCode};

use super::RespResult;
use crate::{get_config, resp_error::RespError, resp_body::RespBody};

#[cfg(feature = "mime")]
#[allow(dead_code)]
static JSON_TYPE: &mime::Mime = &mime::APPLICATION_JSON;

#[allow(dead_code)]
#[allow(clippy::map_identity)]
#[inline]
fn prepare_respond<T, E>(
    r: &RespResult<T, E>,
) -> (Vec<u8>, StatusCode, Option<(HeaderName, HeaderValue)>)
where
    T: RespBody,
    E: RespError,
{
    #[allow(unused_variables)]
    let cfg = &get_config().resp;

    let (body, is_fail) = serde_json::to_vec(r)
        .map(|v| (v, false))
        .map_err(|err| {
            #[cfg(feature = "log")]
            logger::error!("RespResult 响应出现异常 : {}", err);
            err
        })
        .unwrap_or_else(|err| {
            (
                format!(r#"{{"panic-error":"序列化响应体失败","err-msg":{}}}"#, err).into(),
                true,
            )
        });

    let status = match r {
        RespResult::Success(_) => {
            #[cfg(feature = "log")]
            logger::debug!("RespResult 接管的 [成功] 响应",);
            StatusCode::OK
        }
        RespResult::Err(ref e) => {
            #[cfg(feature = "log")]
            logger::debug!(
                "RespResult 接管的 [异常] 响应 | {} => {}",
                std::any::type_name::<E>(),
                e.log_message()
            );
            e.http_code()
        }
    };
    #[cfg(feature = "extra-code")]
    let r_header = {
        match r {
            RespResult::Success(_) => None,
            RespResult::Err(e) => cfg.extra_code.as_ref().map(|n| {
                (
                    n.clone(),
                    HeaderValue::from_str(&e.extra_code().to_string()).expect("Bad HeaderValue"),
                )
            }),
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
    (
        body,
        if !is_fail {
            status
        } else {
            http::StatusCode::INTERNAL_SERVER_ERROR
        },
        r_header,
    )
}

#[derive(Debug, serde::Serialize)]
pub struct Nil;

impl From<()> for Nil {
    fn from(_: ()) -> Self {
        Self
    }
}

impl From<std::convert::Infallible> for Nil {
    fn from(_: std::convert::Infallible) -> Self {
        Self
    }
}

impl std::fmt::Display for Nil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nil")
    }
}
