use std::collections::HashMap;

use chrono::DateTime;
use reqwest::header::HeaderMap;
use bytes::Bytes;

use crate::book::Book;

#[derive(Clone, Debug)]
pub struct Client {
    pub api_url: String,
    client: reqwest::Client
}

impl Client {
    pub fn new(api_url: Option<&str>) -> Self {
        Self {
            api_url: api_url.unwrap_or("https://api.devgoldy.xyz/aghpb").to_string(),
            client: reqwest::Client::new()
        }
    }

    /// Asynchronously grabs a random anime girl holding a programming book.
    /// 
    /// WARNING: Will panic on incorrect category.
    /// 
    /// Uses the ``/v1/random`` endpoint.
    pub async fn random(&self, category: Option<&str>) -> Result<Book, reqwest::Error> {
        let mut queries: Vec<(&str, &str)> = Vec::new();

        if let Some(category) = category {
            queries.push(("category", category));
        }

        let response = self.client.get(self.api_url.clone() + "/v1/random").query(&queries).send().await?;

        if response.status().is_success() {
            let headers = response.headers().to_owned();
            let bytes = response.bytes().await?;

            Ok(get_book(headers, bytes))
        } else {
            Err(panic_on_api_error(&response.text().await?))
        }

    }

    /// Asynchronously grabs list of available categories.
    /// 
    /// Uses the ``/v1/categories`` endpoint.
    pub async fn categories(&self) -> Result<Vec<String>, reqwest::Error> {
        let mut base_url = self.api_url.clone();

        base_url.push_str("/v1/categories");

        let res = self.client.get(base_url).send().await?;
        let json: Vec<String> = serde_json::from_str(&res.text().await?).expect("Failed to deserialize json response");

        Ok(json)
    }
}


fn get_book(headers: HeaderMap, bytes: Bytes) -> Book {
    let name = headers.get("book-name").expect("Failed acquiring book name header!").to_str().expect(
        "Failed converting book name to string."
    ).to_owned();

    let category = headers.get("book-category").expect("Failed acquiring book category header!").to_str().expect(
        "Failed converting book category to string.").to_owned();

    let date_added = DateTime::parse_from_str(headers.get("book-date-added").expect(
        "Failed acquiring book date added header!"
    ).to_str().expect("Failed converting book date time to string."), "%Y-%m-%d %H:%M:%S%z").expect(
        "Failed to convert book's date added header to date time object."
    );

    Book {
        name,
        category,
        date_added,
        raw_bytes: bytes
    }
}

fn panic_on_api_error(text: &String) -> reqwest::Error {
    let error_json: HashMap<String, String> = serde_json::from_str(text).unwrap();
    panic!("API Error: {:?}", error_json.get("message").unwrap());
}