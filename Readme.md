# Resp Result

Help data structure for web framework response

[![Github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/Goodjooy/resp-result)
[![Crates.io](https://img.shields.io/crates/v/resp-result.svg?style=for-the-badge)](https://crates.io/crates/resp-result)
![Licente](https://img.shields.io/github/license/Goodjooy/resp-result?style=for-the-badge)

## Why

- `Result` will become `500` using as a web framework response type when `Err(_)`, the action usually not I expect
- using a non-Result type as a web framework response type cannot use `?`, the code will fill with `if let` or `match`

that why I need a `RespResult`, which can

- control respond code or other messages when it becomes `RespResult::Err`, not always `500`
- impl the [`Try`](std::ops::Try) thus can use friendly `?` to simplify code

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
- `nightly_try_v2` : impl `Try` for `RespResult` making it can use `?`, it will enable feature [try_trait_v2](https://github.com/rust-lang/rust/issues/84277) and require **Nightly** rust compiler

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
/// this can be used as a handler return type
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
/// this can be used as a handler return type
type PlainRResult<T> = RespResult<T, PlainError>;

pub async fn welcome_short_name(name: String) -> PlainRResult<String>{
    if name.len() >= 8{
        // you can use `?` just like the function that returns `Result`
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

In general the `RespResult::Success` is always generate response with status code `200 OK` and using [`serde_json`](https://crates.io/crates/serde_json) 
serialize the body into json. But sometimes we want to return an `304 Not Modified` with empty body to tell the client 
the resource does not change. To support above using a situation, comes out the `ExtraFlag` and `ExtraFlags`

#### Extra Flag

extra flag have 4 different types can bring different effects on response

- `empty_body`: this flag will stop `RespResult` perform serialize into response body
- `status`: this flag will overwrite `StatusCode` of response
- `set-header`: this flag will insert or append provide header into the response header map
- `remove-header`: this flag will remove header from the response header map

different extra flags can use `+` to combine effect or `+=` to adding effect

#### Extra Flags

extra flags is a set of extra flag

#### FlagWrap

flag wrap provides a wrap to send the extra flag

when using the extra flag, you need changing the return type from `RespResult<T, E>` to `RespResult<FlagWrap<T>, E>`

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
/// this can be used as the handler return type
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
            // using `flag_ok` directly construct a flag with the resp result
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

for example, by config, we can change the response body into following

```json
{
  "status": "fail",
  "reterror": 10001,
  "message": "something wrong",
  "body": null
}
```

See the doc of [`ConfigTrait`](self::config::ConfigTrait) for more information

### Help Macros

#### `resp_result` attribute macro

This macro is used on the function. It will convert the original [`Result<T,E>`](std::result::Result) into the [`RespResult`](crate::RespResult),
this makes writing handler more convenience. 
> Note: require `E` in `Result` implement the [`RespError`](crate::RespError)
- example 
```rust
// the `rresult` is an alias of `resp_result`
// the function `test` now will return a `RespResult`
#[rresult]
fn test((a, b): (i32, i64), foo: String) -> Result<(), PlainError> {
    println!("{a},{b},{foo}");
    let a = foo.parse::<i32>()?;
    println!("{a:?}");
    Ok(())
}
```

#### `RespError` derive macro

Using this macro while implement [`RespError`](RespError) for the enum, usually using with [`thiserror`](thiserror::Error)

It now has 2 arg on each variant of enum
1. `err_msg` : the message return to the client, usually need to erase the sensitive message compare with `log_message`. if not provided it will using `log_message`
2. `err_code`: the Http Status Code returned by this kind of error. If not provide, will be 500

Here is an example

```rust
    use std::num::ParseIntError;

    use axum::extract::rejection::PathRejection;
    use axum_resp_result::RespError;
    #[derive(Debug, thiserror::Error, RespError)]
    pub(super) enum PlainError {
        #[error("Parse Request Path Error: {0}")]
        #[resp_result(
            err_msg = "Parse Request Path Error", 
            // the error_code can either the code number or the code name
            err_code = 400
        )]
        Path(#[from] PathRejection),
        #[error("Parse Int Error: {0}")]
        #[resp_result(err_msg = "Invalid Input Integer", err_code = "Bad Request")]
        ParseInt(#[from] ParseIntError),
    }
```