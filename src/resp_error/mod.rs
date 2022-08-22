use std::borrow::Cow;

/// the error when [`RespResult`](crate::RespResult) is `Err(_)`
pub trait RespError {
    /// message for logger
    fn log_message(&self) -> Cow<'_, str>;
    /// message for response
    ///
    /// ## Default
    /// the [`RespError::resp_message`] default is equal to [`RespError::log_message`]
    fn resp_message(&self) -> Cow<'_, str> {
        self.log_message()
    }

    /// the http code of this error
    ///
    /// ## Default
    /// the default http code is `500 Internal Server Error`
    fn http_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }

    #[cfg(feature = "extra-code")]
    /// the associate type of extra message
    type ExtraMessage: serde::Serialize + 'static + Sized + std::fmt::Display;
    #[cfg(feature = "extra-code")]
    /// get the extra message
    fn extra_message(&self) -> Self::ExtraMessage;
}
