use std::marker::PhantomData;

use crate::RespResult;

use super::{NonExtra, RespExtra};

pub struct ExtraRespResult<T, E, Extra: RespExtra = NonExtra> {
    inner: RespResult<T, E>,
    __extra: PhantomData<Extra>,
}
