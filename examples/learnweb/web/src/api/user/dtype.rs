use poem_openapi::{payload::Json, Tags, ApiResponse, Object};

#[derive(Debug, Object, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct UserInfo {
    // #[oai(max_length = 128)]
    pub username: Option<String>,
    // #[oai(max_length = 128)]
    pub useremail: Option<String>,
    // #[oai(max_length = 128)]
    pub usercode: Option<String>,
}


#[derive(Tags)]
pub enum UserApiTags {
    UserInfo
}

#[derive(ApiResponse)]
pub enum UserInfoResp {
    #[oai(status = 200)]
    Ok(Json<Vec<UserInfo>>),
    #[oai(status = 400)]
    NotFound
}