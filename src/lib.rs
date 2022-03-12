#![feature(try_trait_v2)]
mod config;
mod resp_error;

use config::ConfigTrait;

mod resp_result;

static RESP_RESULT_CONFIG: state::Storage<&'static dyn ConfigTrait> = state::Storage::new();

pub fn set_config<C: ConfigTrait>(cfg: C) {
    let boxed = Box::new(cfg) as Box<dyn ConfigTrait>;
    let ref_box = Box::leak::<'static>(boxed) as &dyn ConfigTrait;
    let rsp = RESP_RESULT_CONFIG.set(ref_box);
    if !rsp {
        panic!("Resp Result 配置已经被设置了")
    }
}

pub(crate) fn get_config() -> &'static dyn ConfigTrait {
    if let Some(cfg) = RESP_RESULT_CONFIG.try_get() {
        *cfg
    } else {
        panic!("Resp Result未设置任何配置文件")
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        extract::OriginalUri,
        routing::{get, post},
        Router, Server,
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_roudt() {
        let route = Router::new().route("/aa", get(|| async { "aa" })).route(
            "/aa",
            post(|uri: OriginalUri| async move {
                println!("POST,{:?}", uri);
                "aa"
            }),
        );

        Server::bind(&"127.0.0.1:6000".parse().unwrap())
            .serve(route.into_make_service())
            .await
            .unwrap();
    }
}
