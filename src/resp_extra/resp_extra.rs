pub trait RespExtra: Sized {
    #[cfg(all(feature = "for-actix", not(feature = "for-axum")))]
    /// acitx 框架响应的附加消息添加
    fn actix_extra(&self, _resp: &mut actix_web::HttpResponseBuilder) {}

    #[cfg(all(feature = "for-axum", not(feature = "for-actix")))]
    /// axum 框架响应的附加消息添加
    fn axum_extra(&self, resp: http::response::Builder) -> http::response::Builder {
        resp
    }
}

impl<T> RespExtra for T where T: serde::Serialize + 'static {}
