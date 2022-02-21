// 
// use sqlx::{FromRow};

use crate::sys::db;

use crate::api::user::dtype::UserInfo;

// 

// #[derive(Debug, FromRow)]
// pub struct User {user_code: Option<String>}
pub async fn get_name() -> Result<Vec<UserInfo>, sqlx::Error> {
    let pools = db::db_pool();
    if let Some(pool) = pools {
        Ok(sqlx::query_as!(UserInfo, r#"SELECT usercode, username, useremail from public.user"#,).fetch_all(pool).await?)
    } else {
        Ok(vec![ UserInfo {usercode:None, username: None, useremail: None }])
    }

}