mod extra_resp_result;
pub trait RespExtra {
    #[cfg(feature = "for-actix")]
    /// acitx 框架响应的附加消息添加
    fn actix_extra(_resp: &mut actix_web::HttpResponseBuilder) {}

    #[cfg(feature = "for-axum")]
    /// axum 框架响应的附加消息添加
    fn axum_extra(resp: http::response::Builder) -> http::response::Builder {
        resp
    }
}

pub struct NonExtra;

impl RespExtra for NonExtra {}
