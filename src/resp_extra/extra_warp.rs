use crate::{RespError, RespResult};

use super::{resp_extra::RespExtra, serde_data::LoadSerde, RespBody};

pub struct ExtraWrap<T, E: super::resp_extra::RespExtra> {
    data: T,
    extra: E,
}

impl<T: RespError, E> RespError for ExtraWrap<T, E>
where
    E: super::resp_extra::RespExtra,
{
    fn description(&self) -> std::borrow::Cow<'static, str> {
        self.data.description()
    }
    #[cfg(feature = "extra-code")]
    type ExtraCode = T::ExtraCode;
    #[cfg(feature = "extra-code")]
    fn extra_code(&self) -> Self::ExtraCode {
        self.data.extra_code()
    }

    fn http_code(&self) -> http::StatusCode {
        self.data.http_code()
    }
}

impl<T, E> RespBody for ExtraWrap<T, E>
where
    T: serde::Serialize,
    E: super::resp_extra::RespExtra,
{
}
impl<T, E> LoadSerde for ExtraWrap<T, E>
where
    T: serde::Serialize,
    E: super::resp_extra::RespExtra,
{
    type SerdeData = T;

    fn load_serde(&self) -> &Self::SerdeData {
        &self.data
    }
}

#[cfg(all(feature = "for-actix", not(feature = "for-axum")))]
impl<T, E> RespExtra for ExtraWrap<T, E>
where
    E: super::resp_extra::RespExtra,
{
    fn actix_extra(&self, resp: &mut actix_web::HttpResponseBuilder) {
        self.extra.actix_extra(resp)
    }
}

#[cfg(all(feature = "for-axum", not(feature = "for-actix")))]
impl<T, E> RespExtra for ExtraWrap<T, E>
where
    E: super::resp_extra::RespExtra,
{
    fn axum_extra(&self, resp: http::response::Builder) -> http::response::Builder {
        self.extra.axum_extra(resp)
    }
}

impl<T, E> RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
{
    pub fn map_extra<Ex>(self, extra: Ex) -> RespResult<ExtraWrap<T, Ex>, E>
    where
        Ex: super::resp_extra::RespExtra,
    {
        self.map(|data| ExtraWrap { data, extra })
    }

    pub fn err_extra<Ex>(self, extra: Ex) -> RespResult<T, ExtraWrap<E, Ex>>
    where
        Ex: super::resp_extra::RespExtra,
    {
        self.map_err(|error| ExtraWrap { data: error, extra })
    }
}
