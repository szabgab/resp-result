mod extra_warp;
mod resp_extra;
mod serde_data;

pub use extra_warp::ExtraWrap;
pub use {resp_extra::RespExtra, serde_data::LoadSerde};

pub trait RespBody: resp_extra::RespExtra + serde_data::LoadSerde {}

impl<T> RespBody for T where T: serde::Serialize + 'static {}

pub struct AdHoc<F>(
    #[cfg(all(feature = "for-actix", not(feature = "for-axum")))] F,
    #[cfg(all(feature = "for-axum", not(feature = "for-actix")))] F,
);

#[cfg(all(feature = "for-actix", not(feature = "for-axum")))]
impl<F> RespExtra for AdHoc<F>
where
    F: Fn(&mut actix_web::HttpResponseBuilder) + 'static,
{
    fn actix_extra(&self, resp: &mut actix_web::HttpResponseBuilder) {
        self.0(resp)
    }

    #[cfg(all(feature = "for-axum", not(feature = "for-actix")))]
    fn axum_extra(self, resp: http::response::Builder) -> http::response::Builder {
        resp
    }
}

#[cfg(all(feature = "for-axum", not(feature = "for-actix")))]
impl<F> RespExtra for AdHoc<F>
where
    F: Fn(http::response::Builder) -> http::response::Builder,
{
    fn axum_extra(&self, resp: http::response::Builder) -> http::response::Builder {
        (&self.0)(resp)
    }
}

impl<F> AdHoc<F> {
    #[cfg(all(feature = "for-actix", not(feature = "for-axum")))]
    pub fn new(func: F) -> Self
    where
        F: Fn(&mut actix_web::HttpResponseBuilder),
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
