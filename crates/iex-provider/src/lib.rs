pub mod models;
pub mod provider;
mod request;

use lazy_static::lazy_static;
use std::env::var;

lazy_static! {
    pub static ref IEX_API_KEY: String = var("IEX_API_KEY").unwrap();
}

pub const IEX_BASE_URL: &str = "https://cloud.iexapis.com/stable";
