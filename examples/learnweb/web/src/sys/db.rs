
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

pub async fn init_db_pool() -> Result<(), Box<dyn std::error::Error>>{

    if let Ok(uri) = &env::var("PG_DB_URL") {    
        println!("uri: {}", uri);
        let pool = PgPoolOptions::new()
        .after_connect(| _conn| Box::pin(async move {
            println!("connect succeed");
            Ok(())
         }))
        .min_connections(1)
        .max_connections(20)
        .connect(uri).await?;
        assert!(DATABASE_POOL.set(pool).is_ok());
        Ok(())
    } else {
        println!("database failed");
        Err(Box::new(SuperError { side: SuperErrorSideKick }))
    }

}

#[allow(dead_code)]
pub fn db_pool() -> Option<&'static Pool<Postgres>> {
    DATABASE_POOL.get()
}

