use self::serde::SerdeConfig;

mod serde;

pub trait ConfigTrait: Sync + 'static
where
    Self: SerdeConfig,
{
}
