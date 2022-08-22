use std::{
    convert::{self, Infallible},
    ops::{ControlFlow, FromResidual, Try},
};

use crate::RespError;

use super::RespResult;

impl<T, E: RespError> Try for RespResult<T, E> {
    type Output = T;

    type Residual = RespResult<convert::Infallible, E>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }
    #[inline]
    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            RespResult::Success(data) => {
                #[cfg(feature = "log")]
                logger::debug!("RespResult ControlFlow Continue");
                ControlFlow::Continue(data)
            }
            RespResult::Err(e) => {
                #[cfg(feature = "log")]
                logger::error!("RespResult ControlFlow Break : `{}`", &e.log_message());
                ControlFlow::Break(RespResult::Err(e))
            }
        }
    }
}

impl<T, E, Ei> FromResidual<RespResult<Infallible, Ei>> for RespResult<T, E>
where
    E: From<Ei>,
{
    #[inline]
    fn from_residual(residual: RespResult<Infallible, Ei>) -> Self {
        match residual {
            RespResult::Err(e) => Self::Err(From::from(e)),
            RespResult::Success(_) => unreachable!(),
        }
    }
}

impl<T, E, F: From<E>> FromResidual<Result<Infallible, E>> for RespResult<T, F> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Err(e) => Self::Err(F::from(e)),
            Ok(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{RespError, RespResult};

    struct A;
    struct B;

    impl From<A> for MockA {
        fn from(a: A) -> Self {
            MockA::A(a)
        }
    }

    impl From<B> for MockA {
        fn from(v: B) -> Self {
            MockA::B(v)
        }
    }

    enum MockA {
        A(A),
        B(B),
    }
    impl RespError for MockA {
        fn log_message(&self) -> std::borrow::Cow<'static, str> {
            "MockA".into()
        }

        #[cfg(feature = "extra-code")]
        type ExtraMessage = String;
        #[cfg(feature = "extra-code")]
        fn extra_message(&self) -> Self::ExtraMessage {
            String::new()
        }
    }

    // test wether ? can work on Result
    fn _testb() -> RespResult<u32, MockA> {
        let a = Result::<_, A>::Ok(11u32)?;
        let _b = RespResult::<_, MockA>::ok(a)?;
        let c = Result::<u32, B>::Err(B)?;

        RespResult::Success(c)
    }
}
