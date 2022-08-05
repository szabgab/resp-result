mod actix;
pub mod axum;

#[allow(unused_imports)]
use std::str::FromStr;

use http::{
    header::{HeaderName, CONTENT_TYPE},
    HeaderMap, HeaderValue, StatusCode,
};

use super::RespResult;
use crate::{get_config, resp_body::RespBody, resp_error::RespError};

#[cfg(feature = "mime")]
#[allow(dead_code)]
static JSON_TYPE: &mime::Mime = &mime::APPLICATION_JSON;

struct PrepareRespond {
    pub(crate) body: Vec<u8>,
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
}

impl PrepareRespond {
    #[allow(dead_code)]
    #[allow(clippy::map_identity)]
    #[inline]
    pub fn from_resp_result<T, E>(resp: &RespResult<T, E>) -> Self
    where
        T: RespBody,
        E: RespError,
    {
        let mut this = Self {
            body: Vec::new(),
            status: StatusCode::OK,
            headers: HeaderMap::new(),
        };

        #[allow(unused_variables)]
        let cfg = &get_config().resp;

        this.serde_body(resp);

        this.set_status(resp);

        this.set_header(resp, cfg.extra_code.as_ref());

        #[cfg(feature = "log")]
        logger::info!(
            "RespResult 响应 准备构造 Status :{} BodySize: {}",
            this.status,
            this.body.len()
        );

        this
    }

    fn serde_body<T, E>(&mut self, resp: &RespResult<T, E>)
    where
        T: RespBody,
        E: RespError,
    {
        let body = serde_json::to_vec(resp)
            .map_err(|err| {
                #[cfg(feature = "log")]
                logger::error!("RespResult 响应出现异常 : {}", err);
                err
            })
            .expect("Json 响应时序列化异常");

        let _ = std::mem::replace(&mut self.body, body);
    }

    fn set_header<T, E>(&mut self, resp: &RespResult<T, E>, extra_header: Option<&HeaderName>)
    where
        T: RespBody,
        E: RespError,
    {
        self.headers.append(
            CONTENT_TYPE,
            HeaderValue::try_from(JSON_TYPE.as_ref()).expect("Bad HeaderValue"),
        );
        // extra header
        #[cfg(feature = "extra-code")]
        match (resp, extra_header) {
            (RespResult::Success(_), _) | (_, None) => (),
            (RespResult::Err(err), Some(key)) => {
                self.headers.append(
                    key,
                    HeaderValue::from_str(&err.extra_code().to_string()).expect("Bad HeaderValue"),
                );
            }
        }
    }

    fn set_status<T, E>(&mut self, resp: &RespResult<T, E>)
    where
        T: RespBody,
        E: RespError,
    {
        // status code
        let status = match resp {
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

        self.status = status
    }
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
