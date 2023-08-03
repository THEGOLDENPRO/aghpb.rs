use crate::book::Book;

#[derive(Clone, Debug)]
pub struct Client {
    pub api_url: String,
    client: reqwest::Client
}

impl Client {
    pub fn new(api_url: Option<String>) -> Self {
        Self {
            api_url: api_url.unwrap_or("https://api.devgoldy.xyz/aghpb".to_owned()),
            client: reqwest::Client::new()
        }
    }

    /// Asynchronously grabs a random anime girl holding a programming book.
    /// 
    /// Uses the ``/v1/random`` endpoint.
    pub async fn random(&self, category: Option<&str>) -> Result<Book, reqwest::Error> {
        let mut base_url = self.api_url.clone();

        base_url.push_str("/v1/random");

        if let Some(category) = category {
            base_url.push_str(format!("?category={}", category).as_str());
        }

        let res = self.client.get(base_url).send().await?;
        let headers = res.headers();

        Ok(Book {
            name: headers.get("book-name").expect("Failed acquiring book name").to_str().expect("Failed converting book name to string").to_owned(),
            category: category.unwrap_or(headers.get("book-category").expect("Failed acquiring book category").to_str().expect("Failed converting book category to string")).to_owned(),
            image: image::load_from_memory(&res.bytes().await?).expect("Failed to load image")
        })
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
