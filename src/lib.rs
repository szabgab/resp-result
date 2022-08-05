#![feature(try_trait_v2)]
mod config;
mod convert;
mod owner_leak;
mod resp_error;
mod resp_extra;
mod resp_result;

use once_cell::sync::OnceCell;
#[cfg(feature = "for-axum")]
pub use resp_result::to_response::axum::axum_respond_part;

use config::InnerConfig;
pub use config::{ConfigTrait, DefaultConfig, RespConfig, SerdeConfig};
pub use convert::{IntoRespResult, IntoRespResultWithErr};
pub use resp_error::RespError;
#[cfg(feature = "extra-resp")]
pub use resp_extra::{AdHoc, ExtraWrap};
pub use resp_extra::{ExtraRespExt, RespExtra};
pub use resp_result::{Nil, RespResult};

#[cfg(all(feature = "for-actix", feature = "for-axum"))]
compile_error!("`for-actix` and `for-axum` can not use at the some time");

static RESP_RESULT_CONFIG: OnceCell<InnerConfig> = OnceCell::new();

pub fn set_config<C: ConfigTrait>(cfg: &C) {
    let inner = InnerConfig::from_cfg(cfg);

    if RESP_RESULT_CONFIG.set(inner).is_err() {
        panic!("Resp Result 配置已经被设置了")
    }
}

pub(crate) fn get_config() -> &'static InnerConfig {
    RESP_RESULT_CONFIG.get_or_init(|| {
        #[cfg(feature = "log")]
        logger::warn!("未配置RespResult 配置文件，将使用默认配置");
        Default::default()
    })
}

pub fn leak_string(s: String) -> &'static str {
    let ls = Box::leak(s.into_boxed_str()) as &'static str;
    ls
}
