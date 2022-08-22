mod resp;
mod status_signed;

pub use self::resp::RespConfig;
pub use self::serde::SerdeConfig;
pub use self::status_signed::StatusSign;
use self::{resp::InnerRespConfig, serde::InnerSerdeConfig};

mod serde;

/// the config trait that giving for [`set_config`](crate::set_config)
///
/// this trait is the sub trait of [`SerdeConfig`] and [`RespConfig`]
pub trait ConfigTrait: Sync + 'static
where
    Self: SerdeConfig,
    Self: RespConfig,
{
}

pub(crate) struct InnerConfig {
    pub(crate) serde: InnerSerdeConfig,
    pub(crate) resp: InnerRespConfig,
}

impl Default for InnerConfig {
    fn default() -> Self {
        Self::from_cfg(&DefaultConfig)
    }
}

impl InnerConfig {
    pub(crate) fn from_cfg<C: ConfigTrait>(cfg: &C) -> Self {
        Self {
            serde: InnerSerdeConfig::into_inner(cfg),
            resp: InnerRespConfig::into_inner(cfg),
        }
    }
}

pub struct DefaultConfig;

impl SerdeConfig for DefaultConfig {}

impl RespConfig for DefaultConfig {}

impl ConfigTrait for DefaultConfig {}
