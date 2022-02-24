
use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;


use std::error::Error;
use std::fmt;


#[derive(Debug)]
struct SuperErrorSideKick;

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl Error for SuperErrorSideKick {}

#[derive(Debug)]
struct SuperError {
    side: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}
 
impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.side)
    }
}



/// DATABASE_POOL
pub static DATABASE_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn init_db_pool() -> Result<Option<&'static Pool<Postgres>>, Box<dyn std::error::Error>>{

    // 获取配置的环境变量
    if let Ok(uri) = &env::var("PG_DB_URL") {    
        println!("uri: {}", uri);
        let pool = PgPoolOptions::new()
        .after_connect(| _conn| Box::pin(async move {
            println!("connect succeed");
            // conn.execute("SET application_name = 'your_app';").await?;
            Ok(())
         }))
        .min_connections(1)
        .max_connections(20)
        .connect(uri).await?;
        assert!(DATABASE_POOL.set(pool).is_ok());
        Ok(DATABASE_POOL.get())
    } else {
        println!("database failed");
        Err(Box::new(SuperError { side: SuperErrorSideKick }))
    }

}

#[allow(dead_code)]
pub fn db_pool() -> Option<&'static Pool<Postgres>> {
    DATABASE_POOL.get()
}

pub trait DbSerivce {}
// 创建获取数据库链接的trait; 也可以直接将get_pool方法在本trait中实现
pub trait DbPool {
    fn get_pool() -> Option<&'static Pool<Postgres>>; 
}

// 为所有实现了DbService的struct提供内联获取数据库连接的函数
impl<T> DbPool for T where T: DbSerivce {
    fn get_pool() -> Option<&'static Pool<Postgres>> {
        db_pool()
    }
}

