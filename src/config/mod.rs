pub use self::serde::SerdeConfig;

mod serde;


pub trait ConfigTrait: Sync + 'static
where
    Self: SerdeConfig,
{
}

pub struct  DefaultConfig;

impl SerdeConfig for DefaultConfig {
    
}

impl ConfigTrait for DefaultConfig {
    
}