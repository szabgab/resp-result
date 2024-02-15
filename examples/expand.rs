use error::PlainError;

fn main() {
    let _ret = test((1, 2), String::new());
}
mod error {
    use std::{borrow::Cow, num::ParseIntError};

    use axum::extract::rejection::PathRejection;
    use axum_resp_result::RespError;
    use http::StatusCode;

    pub(super) struct PlainError {
        pub(super) msg: String,
        pub(super) _code: u32,
    }

    impl From<PathRejection> for PlainError {
        fn from(err: PathRejection) -> Self {
            Self::new(err.to_string(), 991)
        }
    }

    impl From<ParseIntError> for PlainError {
        fn from(value: ParseIntError) -> Self {
            Self::new(value.to_string(), 1002)
        }
    }

    impl PlainError {
        pub(super) fn new(msg: String, _code: u32) -> Self {
            Self { msg, _code }
        }
    }

    impl RespError for PlainError {
        fn log_message(&self) -> Cow<'_, str> {
            format!("Plain Error Happened: {}", self.msg).into()
        }

        fn http_code(&self) -> http::StatusCode {
            StatusCode::BAD_REQUEST
        }

        // type ExtraMessage = u32;

        // fn extra_message(&self) -> Self::ExtraMessage {
        //     self.code
        // }

        fn resp_message_default() -> Option<Cow<'static, str>> {
            Some("Success".into())
        }

        // fn extra_message_default() -> Option<Self::ExtraMessage> {
        //     Some(0)
        // }
    }
}
use axum_resp_result::rresult;
#[rresult]
fn test((a, b): (i32, i64), foo: String) -> Result<(), PlainError> {
    println!("{a},{b},{foo}");
    Ok(())
}
