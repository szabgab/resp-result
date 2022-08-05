use crate::extra_flag::effect::Effects;

pub use self::serde_data::LoadSerde;

mod serde_data;
pub trait RespBody: LoadSerde + Effects {}

impl<T> RespBody for T where T: serde::Serialize + 'static {}
