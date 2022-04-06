mod extra_warp;
mod resp_extra;
mod serde_data;
use actix_web::HttpResponseBuilder;

use self::resp_extra::RespExtra;

pub trait RespBody: resp_extra::RespExtra + serde_data::LoadSerde {}

impl<T> RespBody for T where T: serde::Serialize + 'static {}

pub struct AdHoc<F>(
    #[cfg(all(feature = "for-actix", not(feature = "for-axum")))] F,
    #[cfg(all(feature = "for-axum", not(feature = "for-actix")))] F,
);

#[cfg(all(feature = "for-actix", not(feature = "for-axum")))]
impl<F> RespExtra for AdHoc<F>
where
    F: Fn(&mut HttpResponseBuilder)+'static,
{
    fn actix_extra(&self, resp: &mut actix_web::HttpResponseBuilder) {
        let f =&self.0;
        
        f(resp)
    }

    #[cfg(all(feature = "for-axum", not(feature = "for-actix")))]
    fn axum_extra(self, resp: http::response::Builder) -> http::response::Builder {
        resp
    }
}

#[cfg(all(feature = "for-axum", not(feature = "for-actix")))]
impl<F> RespExtra for AdHoc<F>
where
    F: FnOnce(http::response::Builder) -> http::response::Builder,
{
    fn axum_extra(self, resp: http::response::Builder) -> http::response::Builder {
        self.0(resp)
    }
}

impl<F> AdHoc<F> {
    #[cfg(all(feature = "for-actix", not(feature = "for-axum")))]
    pub fn new(func: F) -> Self
    where
        F: Fn(&mut HttpResponseBuilder),
    {
        Self(func)
    }
    #[cfg(all(feature = "for-axum", not(feature = "for-actix")))]
    pub fn new(func: F) -> Self
    where
        F: Fn(http::response::Builder) -> http::response::Builder,
    {
        Self(func)
    }
}


