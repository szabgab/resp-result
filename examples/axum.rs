use axum::routing::get;
use axum::Router;
use axum::Server;
use config::AxumConfig;
use echo::echo_number;
use error::PlainError;
use resp_result::set_config;
use resp_result::RespResult;
use want_304::want_304;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    set_config(&AxumConfig);

    let addr = SocketAddr::try_from(([127, 0, 0, 1], 5000u16)).unwrap();

    let router = Router::new()
        .route("/echo/:num", get(echo_number))
        .route("/want_304", get(want_304));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Server Error");
}

mod error {
    use std::borrow::Cow;

    use resp_result::RespError;

    pub(super) struct PlainError {
        pub(super) msg: String,
    }

    impl RespError for PlainError {
        fn extra_message(&self) -> Self::ExtraMessage {
            100001
        }

        type ExtraMessage = u32;

        fn log_message(&self) -> Cow<'_, str> {
            format!("Plain Error Happend: {}", self.msg).into()
        }
    }
}

type PlainRResult<T> = RespResult<T, PlainError>;

mod echo {
    use axum::extract::Path;
    use serde::Deserialize;

    use crate::{error::PlainError, PlainRResult};

    #[derive(Debug, Deserialize)]
    pub(super) struct Input {
        num: String,
    }

    pub(super) async fn echo_number(Path(Input { num }): Path<Input>) -> PlainRResult<String> {
        let num = num.parse::<i32>();

        match num {
            Ok(num) => Ok(format!("get number {}", num)),
            Err(err) => Err(PlainError {
                msg: format!("parse to num error {}", err),
            }),
        }
        .into()
    }
}

mod want_304 {
    use axum::extract::Query;
    use http::{header::CONTENT_TYPE, StatusCode};
    use resp_result::{ExtraFlag, FlagWrap, RespResult};
    use serde::Deserialize;

    use crate::PlainRResult;

    #[derive(Debug, Deserialize)]
    pub(super) struct Input {
        want: bool,
    }

    pub(super) async fn want_304(
        Query(Input { want }): Query<Input>,
    ) -> PlainRResult<FlagWrap<&'static str>> {
        if !want {
            RespResult::flag_ok("Not a 304", ())
        } else {
            RespResult::ok("304").with_flags(
                ExtraFlag::empty_body()
                    + ExtraFlag::status(StatusCode::NOT_MODIFIED)
                    + ExtraFlag::remove_header(CONTENT_TYPE),
            )
        }
    }
}

mod config {
    use std::borrow::Cow;

    use resp_result::{ConfigTrait, RespConfig, SerdeConfig, SignType, StatusSign};

    pub(super) struct AxumConfig;

    impl SerdeConfig for AxumConfig {
        fn signed_status(&self) -> Option<StatusSign> {
            Some(StatusSign::new("status", SignType::new_str("ok", "fail")))
        }

        fn extra_message(&self) -> Option<Cow<'static, str>> {
            Some("reterror".into())
        }

        fn fixed_field(&self) -> bool {
            true
        }

        fn err_msg_name(&self) -> Cow<'static, str> {
            "message".into()
        }
    }

    impl RespConfig for AxumConfig {
        fn head_extra_code(&self) -> Option<Cow<'static, str>> {
            None
        }
    }

    impl ConfigTrait for AxumConfig {}
}
