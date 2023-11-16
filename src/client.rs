use std::{collections::HashMap, error::Error, fmt};

use crate::book::{Book, BookResult};

#[derive(Clone, Debug)]
pub struct Client {
    pub api_url: String,
    client: reqwest::Client
}

#[derive(Debug)]
struct AGHPBError {
    error: String,
    message: String
}

impl Error for AGHPBError {}

impl fmt::Display for AGHPBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API Error: [{}] {}", self.error, self.message)
    }
}

impl Client {
    pub fn new(api_url: Option<&str>) -> Self {
        Self {
            api_url: api_url.unwrap_or("https://api.devgoldy.xyz/aghpb").to_string(),
            client: reqwest::Client::new()
        }
    }

    /// Grabs a random anime girl holding a programming book.
    /// 
    /// Uses the ``/v1/random`` endpoint.
    pub async fn random(&self, category: Option<&str>) -> Result<Book, Box<dyn Error>> {
        let mut queries: Vec<(&str, &str)> = Vec::new();

        if let Some(category) = category {
            queries.push(("category", category));
        }

        let response = self.client.get(self.api_url.clone() + "/v1/random").query(&queries).send().await?;

        if response.status().is_success() {
            let headers = response.headers().to_owned();
            let bytes = response.bytes().await?;

            Ok(Book::from_response(headers, bytes))
        } else {
            let error_json: HashMap<String, String> = serde_json::from_str(&response.text().await?).unwrap();
            Err(
                AGHPBError {
                    error: error_json.get("error").unwrap().to_string(),
                    message: error_json.get("message").unwrap().to_string()
                }.into()
            )
        }

    }

    /// Grabs list of available categories.
    /// 
    /// Uses the ``/v1/categories`` endpoint.
    pub async fn categories(&self) -> Result<Vec<String>, reqwest::Error> {
        let res = self.client.get(self.api_url.clone() + "/v1/categories").send().await?;
        let json: Vec<String> = serde_json::from_str(&res.text().await?).expect("Failed to deserialize json response!");

        Ok(json)
    }

    /// Allows you to search for anime girls holding programming books.
    /// 
    /// Uses the ``/v1/search`` endpoint.
    pub async fn search(&self, query: &str, category: Option<&str>, limit: Option<u8>) -> Result<Vec<BookResult>, reqwest::Error> {
        let mut queries: Vec<(&str, &str)> = Vec::new();
        queries.push(("query", query));

        if let Some(category) = category {
            queries.push(("category", category));
        }

        if let Some(limit) = limit {
            queries.push(("limit", limit.to_string().as_str()));
        }

        let res = self.client.get(self.api_url.clone() + "/v1/search").query(&queries).send().await?;
        let json: Vec<HashMap<String, String>> = serde_json::from_str(&res.text().await?).expect("Failed to deserialize json response!");

        let mut books: Vec<BookResult> = Vec::new();

        for book_dict in json {
            books.push(
                BookResult::from_json(book_dict)
            );
        }

        Ok(books)
    }
}