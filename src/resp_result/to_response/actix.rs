#[cfg(all(feature = "tracing", feature = "for-actix"))]
use trace as tracing;

#[cfg(feature = "for-actix")]
impl<T, E> actix_web::Responder for crate::RespResult<T, E>
where
    T: crate::resp_body::RespBody,
    E: crate::RespError,
{
    type Body = actix_web::body::BoxBody;

    #[inline]
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        to_actix_resp(&self)
    }
}

#[cfg(feature = "for-actix")]
impl<E> actix_web::ResponseError for crate::RespResult<super::Nil, E>
where
    E: crate::RespError,
{
    fn status_code(&self) -> http::StatusCode {
        match self {
            crate::RespResult::Err(e) => e.http_code(),
            crate::RespResult::Success(_) => http::StatusCode::OK,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        to_actix_resp(self)
    }
}

#[cfg(feature = "for-actix")]
#[cfg_attr(
    feature = "tracing",
    trace::instrument(name = "acitx-into-response", skip_all)
)]
fn to_actix_resp<T, E>(this: &crate::RespResult<T, E>) -> actix_web::HttpResponse
where
    T: crate::resp_body::RespBody,
    E: crate::RespError,
{
    use crate::expect_ext::ExpectExt;

    #[cfg(feature = "tracing")]
    let span = trace::span!(trace::Level::DEBUG, "Prepare Actix-Web Response");
    #[cfg(feature = "tracing")]
    let _enter = span.enter();

    let respond = super::PrepareRespond::from_resp_result(this);
    let mut resp = actix_web::HttpResponse::build(respond.status);

    let mut last_head = None;
    for (k, v) in respond.headers {
        let key = k
            .map(|k| {
                last_head.replace(k.clone());
                k
            })
            .or_else(|| last_head.clone())
            .with_expect("Unknown Header key");

        resp.append_header((key, v));
    }

    resp.body(respond.body)
}
