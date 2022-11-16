use std::fmt::Debug;

pub(crate) trait ExpectExt<T, E> {
    fn with_expect(self, msg: &str) -> T
    where
        E: Debug;

    fn with_expect_err(self, msg: &str) -> E
    where
        T: Debug;
}

impl<T, E> ExpectExt<T, E> for Result<T, E> {
    #[inline]
    fn with_expect(self, msg: &str) -> T
    where
        E: Debug,
    {
        #[cfg(feature = "tracing")]
        {
            tracing_unwrap::ResultExt::expect_or_log(self, msg)
        }
        #[cfg(not(feature = "tracing"))]
        {
            Result::expect(self, msg)
        }
    }
    #[inline]
    fn with_expect_err(self, msg: &str) -> E
    where
        T: Debug,
    {
        #[cfg(feature = "tracing")]
        {
            tracing_unwrap::ResultExt::expect_err_or_log(self, msg)
        }
        #[cfg(not(feature = "tracing"))]
        {
            Result::expect_err(self, msg)
        }
    }
}

impl<T> ExpectExt<T, ()> for Option<T> {
    fn with_expect(self, msg: &str) -> T
    where
        (): Debug,
    {
        #[cfg(feature = "tracing")]
        {
            tracing_unwrap::OptionExt::expect_or_log(self, msg)
        }
        #[cfg(not(feature = "tracing"))]
        {
            Option::expect(self, msg)
        }
    }

    fn with_expect_err(self, msg: &str)
    where
        T: Debug,
    {
        #[cfg(feature = "tracing")]
        {
            tracing_unwrap::OptionExt::expect_none_or_log(self, msg)
        }
        #[cfg(not(feature = "tracing"))]
        {
            let None = self else {
                    panic!("not None {} {self:?}",msg)
                };
        }
    }
}
