use poem_openapi::{OpenApi, payload::{Json, PlainText}};

use crate::api::user::{ UserApiTags, UserInfo, UserInfoResp, user_service::UserService };

pub struct UserApi;

#[OpenApi]
impl UserApi {

    #[oai(path="/userinfo", method= "post", tag= "UserApiTags::UserInfo")]
    async fn user_info(&self, user: Json<UserInfo>) -> UserInfoResp {
        println!("入参: {:#?}", user);

        // 调用Service的函数
        if let Ok(users) = UserService::get_name().await {
            UserInfoResp::Ok(Json(users))
        } else {
            let user_info = UserInfo {
                username: Some(String::from("jiangkun")),
                usercode: Some(String::from("10000")),
                useremail: Some(String::from("jiangkun@livstyle.cn"))
            };
            UserInfoResp::Ok(Json(vec![user_info]))
        }

    }

    #[oai(path="/username", method= "get")]
    async fn user_name(
        &self
    ) -> PlainText<String> {
        PlainText("hello!".to_string())
    }

}
