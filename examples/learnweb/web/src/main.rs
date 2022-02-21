use std::env;
use dotenv::dotenv;
mod api;
mod app;
mod sys;
fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    app::start()
}