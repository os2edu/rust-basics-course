use poem::{listener::TcpListener, Route};
use poem_openapi::{OpenApiService};

use std::io::Error;
use crate::sys::{db};
use crate::api::{apic::{init}};

#[tokio::main]
pub async fn start() -> Result<(), Error> {

    // 初始化数据库
    if let Ok(_pool) = db::init_db_pool().await {
        println!("数据库启动成功!!!");
    }    
  
    // Create API service
    let api_service = OpenApiService::new(init(), "Hello LivStyle", "1.0.0")
        .server("http://localhost:9000/api");
  
    // Enable the Swagger UI
    let ui = api_service.swagger_ui(); // "http://localhost:9000"

    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:9000");

    // Start the server and specify that the root path of the API is /api, and the path of Swagger UI is /
    poem::Server::new(listener)
        .run(Route::new().nest("/api", api_service).nest("/doc", ui))
        .await
}