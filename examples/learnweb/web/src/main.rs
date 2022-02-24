use std::env;
use dotenv::dotenv;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
mod api;
mod app;
mod sys;

fn main() -> Result<(), std::io::Error> {
    // 使用dotenv可以读取 .env设置的环境变量
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    app::start()
}