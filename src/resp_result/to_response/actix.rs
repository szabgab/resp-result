#[cfg(feature = "for-actix")]
impl<T, E> actix_web::Responder for crate::RespResult<T, E>
where
    T: crate::resp_extra::RespBody,
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
fn to_actix_resp<T, E>(this: &crate::RespResult<T, E>) -> actix_web::HttpResponse
where
    T: crate::resp_extra::RespBody,
    E: crate::RespError,
{
    let (body, status, extra_code) = super::prepare_respond(this);
    let mut resp = actix_web::HttpResponse::build(status);

    resp.content_type(super::JSON_TYPE.as_ref());

    match extra_code {
        Some(e_header) => {
            resp.insert_header(e_header);
        }
        None => {}
    }
    if let crate::RespResult::Success(data) = this {
        data.actix_extra(&mut resp)
    }
    resp.body(body)
}
