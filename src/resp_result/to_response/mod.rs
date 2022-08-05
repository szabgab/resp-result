mod actix;
pub mod axum;

#[allow(unused_imports)]
use std::str::FromStr;

use http::{
    header::{HeaderName, CONTENT_TYPE},
    HeaderMap, HeaderValue, StatusCode,
};

use super::{serde::SerializeWrap, RespResult};
use crate::{extra_flag::effect::Effects, get_config, resp_body::RespBody, resp_error::RespError};

#[cfg(feature = "mime")]
#[allow(dead_code)]
static JSON_TYPE: &mime::Mime = &mime::APPLICATION_JSON;

#[derive(Debug)]
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
        if resp.body_effect(&mut self.body) {
            serde_json::to_writer(&mut self.body, &SerializeWrap(resp))
                .map_err(|err| {
                    #[cfg(feature = "log")]
                    logger::error!("RespResult 响应出现异常 : {}", err);
                    err
                })
                .expect("Json 响应时序列化异常");
        }
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
        resp.headers_effect(&mut self.headers);
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
        self.status = resp.status_effect().unwrap_or(status)
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

#[cfg(test)]
mod test {
    use http::StatusCode;

    use crate::{ExtraFlag, RespError, RespResult};

    use super::PrepareRespond;
    struct MockErr;

    impl RespError for MockErr {
        fn log_message(&self) -> std::borrow::Cow<'_, str> {
            "Mock Error".into()
        }

        type ExtraCode = String;

        fn extra_code(&self) -> Self::ExtraCode {
            "Mock".into()
        }
    }
    #[test]
    fn test_prepare_resp() {
        let a = RespResult::<_, MockErr>::Success(12i32).with_flags(
            ExtraFlag::EmptyBody
                + ExtraFlag::status(StatusCode::NOT_MODIFIED)
                + ExtraFlag::insert_header(http::header::ETAG, "1234567890")
                + ExtraFlag::insert_header(http::header::CONTENT_TYPE, mime::TEXT_PLAIN.as_ref())
        );

        let p = PrepareRespond::from_resp_result(&a);

        assert_eq!(p.body.len(), 0);
        assert_eq!(p.status, StatusCode::NOT_MODIFIED);
        println!("{p:#?}")
    }
}
