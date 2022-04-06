#[cfg(feature = "extra-resp")]
mod ad_hoc;
#[cfg(feature = "extra-resp")]
mod extra_warp;
mod resp_extra;
mod serde_data;

#[cfg(feature = "extra-resp")]
pub use ad_hoc::AdHoc;
#[cfg(feature = "extra-resp")]
pub use extra_warp::ExtraWrap;

pub use {resp_extra::RespExtra, serde_data::LoadSerde};

pub trait RespBody: resp_extra::RespExtra + serde_data::LoadSerde {}

impl<T> RespBody for T where T: serde::Serialize + 'static {}
