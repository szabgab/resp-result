use crate::{RespError, RespResult};

/// convert into [`RespResult`](crate::RespResult)
pub trait IntoRespResult<T, E: RespError> {
    fn into_rresult(self) -> RespResult<T, E>;
}

/// convert into [`RespResult`](crate::RespResult) with provide error
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
    #[inline]
    fn into_with_err<Et: Into<E>>(self, err: Et) -> RespResult<T, E> {
        self.ok_or(err).into_rresult()
    }
}
