use actix_web::{web, App, HttpServer};
use resp_result::{set_config, DefaultConfig, FlagRespResult, RespResult};
use simple_log::quick;

use crate::greet::{greet, redirect_greet};

#[actix_web::main]
async fn main() {
    quick!();
    set_config(&DefaultConfig);

    HttpServer::new(|| {
        App::new()
            .route("/greet/{name}", web::get().to(greet))
            .route("/greet_redirect", web::get().to(redirect_greet))
    })
    .bind(("127.0.0.1", 5000))
    .expect("Server start Error")
    .run()
    .await
    .expect("Server Error");
}

mod error {
    use std::borrow::Cow;

    use http::StatusCode;
    use resp_result::RespError;

    pub struct Error(pub String, pub u16, pub StatusCode);

    impl RespError for Error {
        fn log_message(&self) -> Cow<'_, str> {
            format!("Error: {}", self.0).into()
        }

        fn http_code(&self) -> http::StatusCode {
            self.2
        }

        type ExtraMessage = u16;

        fn extra_message(&self) -> Self::ExtraMessage {
            self.1
        }

        fn extra_message_default() -> Option<Self::ExtraMessage> {
            0.into()
        }

        fn resp_message_default() -> Option<Cow<'static, str>> {
            Some("".into())
        }
    }
}

type RRresult<T> = RespResult<T, error::Error>;
type FlagRRresult<T> = FlagRespResult<T, error::Error>;

mod greet {
    use actix_web::web;
    use http::{
        header::{CONTENT_TYPE, LOCATION},
        StatusCode,
    };
    use resp_result::{ExtraFlag, RespResult};
    use serde::Deserialize;

    use crate::{error, FlagRRresult, RRresult};

    #[derive(Debug, Deserialize)]
    pub(super) struct Input {
        pub name: String,
    }

    pub(super) async fn greet(path: web::Path<Input>) -> RRresult<String> {
        let Input { name } = path.into_inner();

        if name.starts_with(|c: char| c.is_digit(10)) {
            Err(error::Error(
                format!("Name[{}] cannot start with number", name),
                400,
                StatusCode::BAD_REQUEST,
            ))?;
        }

        RespResult::ok(format!("Welcome! {}", name))
    }

    pub(super) async fn redirect_greet(query: web::Query<Input>) -> FlagRRresult<()> {
        let Input { name } = query.into_inner();
        if name.starts_with(|c: char| c.is_digit(10)) {
            Err(error::Error(
                format!("Name[{}] cannot start with number", name),
                400,
                StatusCode::BAD_REQUEST,
            ))?;
        }

        //redirect
        RespResult::flag_ok(
            (),
            ExtraFlag::empty_body()
                + ExtraFlag::remove_header(CONTENT_TYPE)
                + ExtraFlag::status(StatusCode::FOUND)
                + ExtraFlag::insert_header(LOCATION, format!("/greet/{}", name)),
        )
    }
}
