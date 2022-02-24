pub mod user;
pub mod job;

use poem_openapi::{payload::{PlainText}, OpenApi, param::Query};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn api(
        &self,
        name: Query<Option<String>>,
    ) -> PlainText<String> {
        println!("访问路径v =================  /");
        match name.0 {
            Some(name) => {
                PlainText(format!("hello, {}!", name))
            },
            None => PlainText("hello!".to_string()),
        }
    }
}

pub mod apic {
    // 将父模块的信息引入本模块
    use super::*;
    use user::{user_controller::UserApi};
    pub fn init() -> (Api, UserApi) {
        (Api, UserApi)
    }
}