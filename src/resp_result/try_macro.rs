

#[macro_export]
macro_rules! rtry {
    { $exp:expr } => {
        match $crate::IntoRespResult::into_rresult($exp.map_err(Into::into)){
            $crate::RespResult::Success(value) => value,
            $crate::RespResult::Err(err) =>{
                return $crate::RespResult::err(err)
            },
        }
    };
}

