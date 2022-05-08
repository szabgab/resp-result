use crate::{RespError, RespResult};

use super::{resp_extra::RespExtra, serde_data::LoadSerde, ExtraRespExt, RespBody};

pub struct ExtraWrap<T, E: super::resp_extra::RespExtra> {
    data: T,
    extra: E,
}

impl<T, E: super::resp_extra::RespExtra> ExtraWrap<T, E> {
    pub fn map_to<Ti>(self) -> ExtraWrap<Ti, E>
    where
        Ti: From<T>,
    {
        ExtraWrap {
            data: From::from(self.data),
            extra: self.extra,
        }
    }
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
    fn axum_extra(self, resp: http::response::Builder) -> http::response::Builder {
        self.extra.axum_extra(resp)
    }
}

impl<T, E, Ext> ExtraRespExt<Ext> for RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
    Ext: RespExtra + Clone,
{
    type Output = RespResult<ExtraWrap<T, Ext>, ExtraWrap<E, Ext>>;

    fn mapping_by(self, extra: Ext) -> Self::Output {
        self.map(|data| ExtraWrap {
            data,
            extra: extra.clone(),
        })
        .map_err(|data| ExtraWrap { data, extra })
    }
}

impl<T, E, Ext: RespExtra + Clone> ExtraRespExt<Ext> for Result<T, E> {
    type Output = Result<ExtraWrap<T, Ext>, ExtraWrap<E, Ext>>;

    fn mapping_by(self, extra: Ext) -> Self::Output {
        self.map(|data| ExtraWrap {
            data,
            extra: extra.clone(),
        })
        .map_err(|data| ExtraWrap { data, extra })
    }
}

impl<T, Ext: RespExtra + Clone> ExtraRespExt<Ext> for Option<T> {
    type Output = Option<ExtraWrap<T, Ext>>;

    fn mapping_by(self, extra: Ext) -> Self::Output {
        self.map(|data| ExtraWrap { data, extra })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        resp_extra::resp_extra::DefaultExtra, ExtraRespExt, ExtraWrap, RespError, RespResult,
    };

    struct A;
    struct B;

    impl From<A> for MockA {
        fn from(a: A) -> Self {
            MockA::A(a)
        }
    }

    impl From<B> for MockA {
        fn from(v: B) -> Self {
            MockA::B(v)
        }
    }

    enum MockA {
        A(A),
        B(B),
    }
    impl RespError for MockA {
        fn description(&self) -> std::borrow::Cow<'static, str> {
            "MockA".into()
        }

        #[cfg(feature = "extra-code")]
        type ExtraCode = String;
        #[cfg(feature = "extra-code")]
        fn extra_code(&self) -> Self::ExtraCode {
            String::new()
        }
    }
    #[cfg(feature = "extra-resp")]
    impl crate::resp_extra::RespExtra for MockA {}

    fn _test() -> RespResult<ExtraWrap<u32, DefaultExtra>, ExtraWrap<MockA, DefaultExtra>> {
        let data = Result::<_, A>::Ok(32u32);

        let data = data.mapping_by(DefaultExtra).map_err(ExtraWrap::map_to)?;

        Ok(data).into()
    }
}
