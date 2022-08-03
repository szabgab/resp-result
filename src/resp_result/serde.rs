use serde::{ser::SerializeStruct, Serialize};

use crate::{get_config, resp_error::RespError, resp_extra::RespBody};

use super::RespResult;

impl<T, E> Serialize for RespResult<T, E>
where
    T: RespBody,
    E: RespError,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let cfg = &get_config().serde;
        let (ok_size, err_size) = cfg.get_field_size();

        #[cfg(feature = "log")]
        logger::debug!("开始序列化 成功字段 : {} 失败字段：{}", ok_size, err_size);

        let resp = match self {
            RespResult::Success(data) => {
                #[cfg(feature = "log")]
                logger::debug!("序列化成功模式结果");
                let mut body = serializer.serialize_struct("RespResult", ok_size)?;
                if let Some(n) = cfg.signed_base_status {
                    body.serialize_field(n, &true)?;
                }
                if cfg.full_field {
                    #[cfg(feature = "extra-code")]
                    if let Some(ecl) = cfg.extra_code {
                        body.serialize_field(ecl, &Option::<()>::None)?;
                    }
                    body.serialize_field(cfg.err_msg_name, &Option::<()>::None)?;
                }

                body.serialize_field(cfg.body_name, data.load_serde())?;

                body.end()?
            }
            RespResult::Err(err) => {
                #[cfg(feature = "log")]
                {
                    logger::debug!("序列化失败情况结果");
                    logger::error!("错误信息 : {}", err.description())
                }
                let mut body = serializer.serialize_struct("RespResult", err_size)?;
                if let Some(n) = cfg.signed_base_status {
                    body.serialize_field(n, &false)?;
                }
                #[cfg(feature = "extra-code")]
                if let Some(ecl) = cfg.extra_code {
                    body.serialize_field(ecl, &err.extra_code())?;
                }
                body.serialize_field(cfg.err_msg_name, &err.description())?;

                if cfg.full_field {
                    body.serialize_field(cfg.body_name, &Option::<()>::None)?;
                }
                body.end()?
            }
        };
        Ok(resp)
    }
}
