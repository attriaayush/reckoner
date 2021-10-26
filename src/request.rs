use surf::Result;
use surf;

#[derive(Debug)]
struct Response<T> {
    data: T,
}

pub async fn get(url: &str) -> Result<String> {
    let mut response = surf::get(url).await?;
    let result = response.body_string().await?;
    Ok(result)
}
