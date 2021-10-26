pub mod models;
pub mod provider;
mod request;

pub const IEX_BASE_URL: &str = "https://cloud.iexapis.com/stable";
pub const IEX_API_KEY: &str = env!("IEX_API_KEY");
