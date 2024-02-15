# Resp Result

Help data structure for web framework response

[![Github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/Goodjooy/resp-result)
[![Crates.io](https://img.shields.io/crates/v/resp-result.svg?style=for-the-badge)](https://crates.io/crates/resp-result)
![Licente](https://img.shields.io/github/license/Goodjooy/resp-result?style=for-the-badge)

## Why

- `Result` will become `500` using as web framework response type when `Err(_)`, the action usually not I expect
- using non-Result type as web framework response type cannot using `?`, the code will fill with `if let` or `match`

that why I need a `RespResult`, which can

- control respond code or other message when it become `RespResult::Err`, not always `500`
- impl the [`Try`](std::ops::Try) thus can using friendly `?` to simplify code

> note: because the [`Try`](std::ops::Try) not stable yet, this crate need `Nightly` rust

## Usage

### Install

add `resp-result` into your crate

```toml
[dependencies]
resp-result = "*"
```

#### feature flags

- `extra-error`: enable extra error message in trait `RespError`
- `log`: make [tracing](https://docs.rs/tracing/latest/tracing/) also logger to the [log](https://docs.rs/log/0.4.6/log/)
- `tracing` : enable recorder using [tracing](https://docs.rs/tracing/latest/tracing/)
- `nightly_try_v2` : impl `Try` for `RespResult` making it can using `?`, it will enable feature [try_trait_v2](https://github.com/rust-lang/rust/issues/84277) and require **Nightly** rust compiler

### Define an Error

`RespResult<T,E>` require the `E` impl the `RespError`

for example

```rust
use resp_result::{RespError, RespResult};
use std::borrow::Cow;
use http::StatusCode;

pub struct PlainError(String);

impl RespError for PlainError{
    fn log_message(&self) -> Cow<'_, str> {
        Cow::Owned(format!("PlainError: {}", self.0))
    }

    fn resp_message(&self) -> Cow<'_, str> {
        "PlainError".into()
    }

    fn http_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    type ExtraMessage = String;

    fn extra_message(&self) -> Self::ExtraMessage {
            self.0.clone()
    }
}
/// this can be use as handler return type
type PlainRResult<T> = RespResult<T, PlainError>;
```

### Bound of `T` in `RespResult<T, E>`

The `T` require implement [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and has `'static` lifetime

### Using it

the following is an example for using [`RespResult`]

```rust
use resp_result::{RespError, RespResult};
use std::borrow::Cow;
use http::StatusCode;

pub struct PlainError(String);

impl RespError for PlainError{
    fn log_message(&self) -> Cow<'_, str> {
        Cow::Owned(format!("PlainError: {}", self.0))
    }

    type ExtraMessage = String;

    fn extra_message(&self) -> Self::ExtraMessage {
            self.0.clone()
    }
}
/// this can be use as handler return type
type PlainRResult<T> = RespResult<T, PlainError>;

pub async fn welcome_short_name(name: String) -> PlainRResult<String>{
    if name.len() >= 8{
        // you can using `?` just like the function that return `Result`
        Err(PlainError("the name size great then 8".to_string()))?;
    }

    if name.len() >= 4 {
        // `Result::Ok` can convert into `RespResult::Success` just using `into`
        Ok(format!("welcome! {name} with size great then 4")).into()
    }else{
        // or just direct using `RespResult::ok`
        RespResult::ok(format!("welcome! {name}"))
    }
}
```

### ExtraFlag and ExtraFlags

In general the `RespResult::Success` is always generate response with status code `200 OK` and using [`serde_json`](https://crates.io/crates/serde_json) serialize the body into json. But sometimes we want return an
`304 Not Modified` with empty body to tell the client the resource do not change. To support above using situation, comes out the `ExtraFlag` and `ExtraFlags`

#### Extra Flag

extra flag have 4 different type can bring different effect on response

- `empty_body`: this flag will stop `RespResult` perform serialize into response body
- `status`: this flag will overwrite `StatusCode` of response
- `set-header`: this flag will insert or append provide header into response header map
- `remove-header`: this flag will remove header from response header map

different extra flags can using `+` to combine effect or `+=` to adding effect

#### Extra Flags

extra flags is a set of extra flag

#### FlagWrap

flag wrap provide a wrap to send extra flag

when using extra flag, you need change return type from `RespResult<T, E>` to `RespResult<FlagWrap<T>, E>`

the follow example change Status Code to `404 Not Found`

```rust
use resp_result::{RespError, RespResult, FlagWrap, ExtraFlag};
use std::borrow::Cow;
use http::StatusCode;

pub struct PlainError(String);

impl RespError for PlainError{
    fn log_message(&self) -> Cow<'_, str> {
        Cow::Owned(format!("PlainError: {}", self.0))
    }

    type ExtraMessage = String;

    fn extra_message(&self) -> Self::ExtraMessage {
            self.0.clone()
    }
}
/// this can be use as handler return type
type PlainRResult<T> = RespResult<T, PlainError>;

pub async fn welcome_short_name(
    name: String,
    ) -> PlainRResult<FlagWrap<String>>{
        if name.len() >= 8{
            RespResult::ok(
                format!("welcome! {name} your name size is {}",name.len()))
                // using `with_flags` to covert RespResult<T, E>
                // to `RespResult<FlagWrap<T>, E>`
                // using `()` for no extra flags
                .with_flags(())
        }else{
            // suing `flag_ok` direct construct a flag with resp result
            RespResult::flag_ok(
                format!("Welcome! {name}"),
                ExtraFlag::status(StatusCode::NOT_FOUND)
            )
        }
    }
```

### Effect `RespResult` behavior

by default the `RespResult` will serialize the response body like that

```json
{
  "is-ok": true,
  "error-message": "...",
  "extra-msg": "...",
  "body": null
}
```

the default behavior can be changed by using `set_config` to set global configuration

for example, by config, we can change response body into following

```json
{
  "status": "fail",
  "reterror": 10001,
  "message": "something wrong",
  "body": null
}
```
