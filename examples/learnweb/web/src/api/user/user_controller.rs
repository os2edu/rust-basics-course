use poem_openapi::{OpenApi, payload::{Json, PlainText}};

use crate::api::user::{ UserApiTags, UserInfo, UserInfoResp, user_service::get_name };

pub struct UserApi;

#[OpenApi]
impl UserApi {
    #[oai(path="/userinfo", method= "post", tag= "UserApiTags::UserInfo")]
    async fn user_info(&self, user: Json<UserInfo>) -> UserInfoResp {
        println!("入参: {:#?}", user);

        let user_info = UserInfo {
            username: Some(String::from("jiangkun")),
            usercode: Some(String::from("10000")),
            useremail: Some(String::from("jiangkun@livstyle.cn"))
        };

        if let Ok(users) = get_name().await {
            UserInfoResp::Ok(Json(users))
        } else {
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
