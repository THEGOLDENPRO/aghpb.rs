//! AGHPB API wrapper for 🦀Rust.
//!
//! Copyright (c) 2023-present Goldy
//! 
//! -------------
//! 
//! # Example
//! How to retrieve a random anime girl holding a programming book in Rust.
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

use reqwest::Response;
use std::error::Error;
use image::DynamicImage;

const API_URL: &str = "https://api.devgoldy.xyz/aghpb";

pub struct Book {
    pub name: String,
    pub category: String,
    pub image: DynamicImage,
}

pub async fn random(category: Option<String>) -> Result<Book, Box<dyn Error>> {
    //! Asynchronously grabs a random anime girl holding a programming book.
    //! 
    //! Uses the ``/v1/random`` endpoint.
    let mut url: String = API_URL.to_owned() + "/v1/random";

    let category: String = category.unwrap_or("".to_string());

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