use crate::{RespError, RespResult};

pub trait IntoRespResult<T, E: RespError> {
    fn into_rresult(self) -> RespResult<T, E>;
}

pub trait IntoRespResultWithErr<T, E: RespError> {
    fn into_with_err<Et: Into<E>>(self, err: Et) -> RespResult<T, E>;
}

impl<Te, E, T> IntoRespResult<T, E> for Result<T, Te>
where
    Te: Into<E>,
    E: RespError,
{
    #[inline]
    fn into_rresult(self) -> RespResult<T, E> {
        RespResult::from(self.map_err(|e| e.into()))
    }
}

impl<T, E> IntoRespResultWithErr<T, E> for Option<T>
where
    E: RespError,
{
    fn into_with_err<Et: Into<E>>(self, err: Et) -> RespResult<T, E> {
        self.ok_or(err).into_rresult()
    }
}
