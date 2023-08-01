//! AGHPB API wrapper for 🦀Rust.
//!
//! Copyright (c) 2023-present Goldy
//! 
//! -------------
//! 
//! # Examples
//! ### How to retrieve a random anime girl holding a programming book in Rust.
//! ```rust
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let book = aghpb::random(None).await?;
//! 
//!     println!("Name: {}", book.name);
//!     println!("Category: {}", book.category);
//! 
//!     book.image.save("./anime_girl.png")?;
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ### How to retrieve a list of available categories.
//! ```rust
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let categories = aghpb::categories().await?;
//! 
//!     for category in categories {
//!         println!("{}", category);
//!     }
//! 
//!     Ok(())
//! }
//! ```

use reqwest::Response;
use std::error::Error;
use image::DynamicImage;

const API_URL: &str = "https://api.devgoldy.xyz/aghpb";

pub struct Book {
    pub name: String,
    pub category: String,
    pub image: DynamicImage,
}

/// Asynchronously grabs a random anime girl holding a programming book.
/// 
/// Uses the ``/v1/random`` endpoint.
pub async fn random(category: Option<&str>) -> Result<Book, Box<dyn Error>> {
    let mut url: String = API_URL.to_owned() + "/v1/random";

    let category = category.unwrap_or("");

    if category != "" {
        url.push_str(&("?category=".to_string() + &category));
    }

    let res: Response = reqwest::get(url).await?;
    let headers: &reqwest::header::HeaderMap = res.headers();

    let book = Book {
        name: String::from(headers.get("book-name").unwrap().to_str().unwrap()),
        category: String::from(headers.get("book-category").unwrap().to_str().unwrap()),
        image: image::load_from_memory(&(res.bytes().await?))?,
    };

    Ok(book)
}

/// Asynchronously grabs list of available categories.
/// 
/// Uses the ``/v1/categories`` endpoint.
pub async fn categories() -> Result<Vec<String>, Box<dyn Error>> {
    let url: String = API_URL.to_owned() + "/v1/categories";
    let res: Response = reqwest::get(url).await?;

    let text = res.text().await?;
    let json: Vec<String> = serde_json::from_str(text.as_str()).unwrap();

    Ok(json)
}