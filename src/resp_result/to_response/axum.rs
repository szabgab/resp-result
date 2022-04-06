#[cfg(feature = "for-axum")]
impl<T, E> axum::response::IntoResponse for crate::RespResult<T, E>
where
    T: crate::resp_extra::RespBody,
    E: crate::RespError,
{
    #[inline]
    fn into_response(self) -> axum::response::Response {
        let (body, status, eh) = super::prepare_respond(&self);
        let builder = axum::response::Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, super::JSON_TYPE.as_ref());

        let builder = match eh {
            None => builder,
            Some((k, v)) => builder.header(k, v),
        };
        let builder = match self {
            crate::RespResult::Success(ref data) => data.axum_extra(builder),
            crate::RespResult::Err(ref err) => err.axum_extra(builder),
        };

        builder
            .body(axum::body::boxed(axum::body::Full::from(body)))
            .expect("RespResult 构造响应时发生异常")
    }
}
