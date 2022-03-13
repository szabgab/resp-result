use serde::{ser::SerializeStruct, Serialize};

use crate::{get_config, resp_error::RespError};

use super::RespResult;

static SIGNED_STATUS: &str = "is-ok";
#[cfg(feature = "extra-code")]
static EXTRA_ERR_CODE: &str = "extra-code";
static ERROR_MESSAGE: &str = "error-message";
static BODY: &str = "body";

impl<T, E> Serialize for RespResult<T, E>
where
    T: Serialize,
    E: RespError,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let cfg = get_config();
        let (mut ok_size, mut err_size) = (1, 1);
        // 简易状态标记
        if cfg.signed_base_status() {
            ok_size += 1;
            err_size += 1;
        }
        //额外的异常码
        #[cfg(feature = "extra-code")]
        if cfg.extra_code_local() {
            if cfg.full_field() {
                ok_size += 1;
            }
            err_size += 1;
        }

        if cfg.full_field() {
            ok_size += 1;
            err_size += 1;
        }

        let resp = match self {
            RespResult::Success(data) => {
                let mut body = serializer.serialize_struct("RespResult", ok_size)?;
                if cfg.signed_base_status() {
                    body.serialize_field(SIGNED_STATUS, &true)?;
                }
                if cfg.full_field() {
                    #[cfg(feature = "extra-code")]
                    if cfg.extra_code_local() {
                        body.serialize_field(EXTRA_ERR_CODE, &Option::<()>::None)?;
                    }
                    body.serialize_field(ERROR_MESSAGE, &Option::<()>::None)?;
                }

                body.serialize_field(BODY, data)?;

                body.end()?
            }
            RespResult::Err(err) => {
                let mut body = serializer.serialize_struct("RespResult", err_size)?;
                if cfg.signed_base_status() {
                    body.serialize_field(SIGNED_STATUS, &true)?;
                }
                #[cfg(feature = "extra-code")]
                if cfg.extra_code_local() {
                    body.serialize_field(EXTRA_ERR_CODE, &err.extra_code())?;
                }
                body.serialize_field(ERROR_MESSAGE, &err.description())?;

                if cfg.full_field() {
                    body.serialize_field(BODY, &Option::<()>::None)?;
                }
                body.end()?
            }
        };
        Ok(resp)
    }
}
