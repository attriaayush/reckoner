use dotenv;
use std::env;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    femme::with_level(femme::LevelFilter::Info);

    let mut app = tide::new();

    app.at("/evaluate/dcf").post(dcf::calculate_dcf);
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

pub fn env_var(key: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(e) => panic!("Cannot read the environment variable {}: {}", key, e),
    }
}

mod provider;
mod request;
mod models;
mod dcf;
