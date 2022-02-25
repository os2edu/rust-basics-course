// 
// use sqlx::{FromRow};
use tracing::{info, instrument};

use crate::sys::db::*;
use crate::api::user::dtype::UserInfo;

pub struct UserService {

}

// 实现了DbSerivce trait的struct可以直接使用Self::get_pool()方法
impl DbSerivce for UserService {}

impl UserService {
    #[instrument]
    pub async fn get_name() -> Result<Vec<UserInfo>, sqlx::Error> {
        let pools = Self::get_pool(); // db::db_pool();
        if let Some(pool) = pools {
    
            let sql_text = "SELECT usercode, username, useremail from public.user";
            let no_macro_query = sqlx::query_as::<_, UserInfo>(sql_text).fetch_all(pool).await;
    
            // ### 使用sqlx的宏进行查询数据库时必须先将数据库和对应的表创建完成并且保证连接的数据库地址正确
            // ### 这样做的主要原因是sqlx使用宏查询时会先对sql进行编译期验证用来检查和匹配sql中用到的表和字段等信息
            // ### 本例子中可以根据doc目录中给出的脚本进行创建表和初始化数据
            // let result = sqlx::query_as!(UserInfo, "SELECT usercode, username, useremail from public.user",).fetch_all(pool).await;
            match no_macro_query {
                Ok(data) => {
                    info!("获取数据库数据成功: {:#?}", data);
                    Ok(data)
                },
                Err(err) => {
                    info!("获取数据库数据失败,原因为: {:#?}", err);
                    Err(sqlx::Error::PoolTimedOut)
                } 
            }
        } else {
            Err(sqlx::Error::PoolTimedOut)
        }
    
    }
}

