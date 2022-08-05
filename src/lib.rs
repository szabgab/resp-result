#![feature(try_trait_v2)]
mod config;
mod convert;
mod owner_leak;
mod resp_body;
mod resp_error;
mod resp_result;

#[cfg(feature = "for-axum")]
pub use self::resp_result::to_response::axum::axum_respond_part;
use once_cell::sync::OnceCell;

use config::InnerConfig;
pub use config::{ConfigTrait, DefaultConfig, RespConfig, SerdeConfig};
pub use convert::{IntoRespResult, IntoRespResultWithErr};
pub use resp_error::RespError;
pub use resp_result::{Nil, RespResult};

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
