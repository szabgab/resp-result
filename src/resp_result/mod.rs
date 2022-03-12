mod to_response;
mod try_op;
mod serde;



pub enum RespResult<T,E> {
    Success(T),
    Err(E)
}

impl<T, E> RespResult<T, E> {
    
}

impl<T, E> From<Result<T,E>> for RespResult<T, E> {
    #[inline]
    fn from(r: Result<T,E>) -> Self {
        match r {
            Ok(data) => Self::ok(data),
            Err(err) => Self::err(err),
        }
    }
}

impl<T, E> RespResult<T, E> {
    #[inline]
    pub fn ok(data:T)->Self{
        Self::Success(data)
    }
    #[inline]
    pub fn err(err:E)->Self{
        Self::Err(err)
    }
}

