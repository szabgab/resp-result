use std::ops::{ControlFlow, FromResidual, Try};

use crate::RespError;

use super::RespResult;

impl<T, E:RespError> Try for RespResult<T, E> {
    type Output = T;

    type Residual = E;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }
    #[inline]
    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            RespResult::Success(data) => {
                #[cfg(feature = "log")]
                logger::info!("RespResult ControlFlow Continue");
                ControlFlow::Continue(data)
            }
            RespResult::Err(e) => {
                #[cfg(feature = "log")]
                logger::error!("RespResult ControlFlow Break : `{}`",&e.description());
                ControlFlow::Break(e)},
        }
    }
}

impl<T, E, Ei: Into<E>> FromResidual<Ei> for RespResult<T, E> {
    #[inline]
    fn from_residual(residual: Ei) -> Self {
        Self::Err(residual.into())
    }
}
