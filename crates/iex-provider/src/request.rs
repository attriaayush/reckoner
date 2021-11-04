use anyhow::Result;
use serde::de::DeserializeOwned;

pub async fn get<T: DeserializeOwned>(url: &str) -> Result<T> {
    let response = reqwest::get(url).await?;
    let result = response.json::<T>().await?;
    Ok(result)
}
