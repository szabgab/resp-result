use crate::{
    resp_body::{LoadSerde, RespBody},
    ExtraFlags, RespResult,
};

use super::effect::{Effects, BodyEffect};

pub struct FlagWarp<T> {
    inner: T,
    flags: ExtraFlags,
}

impl<T> FlagWarp<T> {
    pub fn new(data: T, flags: impl Into<ExtraFlags>) -> Self {
        Self {
            inner: data,
            flags: flags.into(),
        }
    }
}

impl<T, E> RespResult<T, E> {
    pub fn with_flags(self, flags: impl Into<ExtraFlags>) -> RespResult<FlagWarp<T>, E> {
        match self {
            RespResult::Success(data) => RespResult::Success(FlagWarp::new(data, flags)),
            RespResult::Err(err) => RespResult::Err(err),
        }
    }
}

impl<T, E> From<RespResult<T, E>> for RespResult<FlagWarp<T>, E> {
    fn from(inner: RespResult<T, E>) -> Self {
        inner.with_flags(())
    }
}

impl<T: LoadSerde> LoadSerde for FlagWarp<T> {
    type SerdeData = T::SerdeData;

    fn load_serde(&self) -> &Self::SerdeData {
        self.inner.load_serde()
    }
}

impl<T> Effects for FlagWarp<T> {
    fn body_effect(&self, body: &mut Vec<u8>) -> BodyEffect {
        self.flags.body_effect(body)
    }

    fn status_effect(&self) -> Option<http::StatusCode> {
        self.flags.status_effect()
    }

    fn headers_effect(&self, map: &mut http::HeaderMap) {
        self.flags.headers_effect(map)
    }
}

impl<T: LoadSerde> RespBody for FlagWarp<T> {}

#[cfg(test)]
mod test {
    use http::StatusCode;

    use crate::{resp_result::serde::SerializeWrap, ExtraFlag, RespError, RespResult};

    struct MockErr;

    impl RespError for MockErr {
        fn log_message(&self) -> std::borrow::Cow<'_, str> {
            "Mock Error".into()
        }
        #[cfg(feature = "extra-code")]
        type ExtraCode = String;

        #[cfg(feature = "extra-code")]
        fn extra_code(&self) -> Self::ExtraCode {
            "Mock".into()
        }
    }

    #[test]
    fn test_serde() {
        let a = RespResult::<_, MockErr>::Success(12i32).with_flags(
            ExtraFlag::EmptyBody
                + ExtraFlag::status(StatusCode::NOT_MODIFIED)
                + ExtraFlag::insert_header(http::header::ETAG, "1234567890")
                + ExtraFlag::insert_header(
                    http::header::CONTENT_TYPE,
                    mime::TEXT_PLAIN_UTF_8.as_ref(),
                ),
        );

        let s = serde_json::to_string_pretty(&SerializeWrap(&a)).unwrap();

        println!("{s}")
    }
}
