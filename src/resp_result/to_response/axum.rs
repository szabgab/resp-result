#[cfg(feature = "for-axum")]
impl<T, E> axum::response::IntoResponse for crate::RespResult<T, E>
where
    T: serde::Serialize,
    E: crate::RespError,
{
    #[inline]
    fn into_response(self) -> axum::response::Response {
        let (body, status, eh) = super::prepare_respond(&self);
        let builder = axum::response::Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, super::JSON_TYPE.as_ref());

        match eh {
            None => builder,
            Some((k, v)) => builder.header(k, v),
        }
        .body(axum::body::boxed(axum::body::Full::from(body)))
        .expect("RespResult 构造响应时发生异常")
    }
}
