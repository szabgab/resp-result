use std::borrow::Cow;

pub trait RespError {
    /// message for logger
    fn log_message(&self) -> Cow<'_, str>;
    /// message for response
    fn resp_message(&self) -> Cow<'_, str> {
        self.log_message()
    }
    fn http_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }

    #[cfg(feature = "extra-code")]
    type ExtraCode: serde::Serialize + 'static + Sized + std::fmt::Display;
    #[cfg(feature = "extra-code")]
    fn extra_code(&self) -> Self::ExtraCode;
}
