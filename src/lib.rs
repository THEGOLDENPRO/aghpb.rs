use reqwest::Response;
use std::error::Error;

const API_URL: &str = "https://api.devgoldy.xyz/aghpb";

pub async fn random(category: Option<String>) -> Result<Response, Box<dyn Error>> {
    let mut url: String = API_URL.to_owned() + "/v1/random";

    let category: String = category.unwrap_or("".to_string());

    if category != "" {
        url.push_str(&("?category=".to_string() + &category));
    }

    let res: Response = reqwest::get(url).await?;

    Ok(res)
}