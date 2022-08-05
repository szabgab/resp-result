mod serde_data;
pub trait RespBody: serde_data::LoadSerde {}

impl<T> RespBody for T where T: serde::Serialize + 'static {}
