//! AGHPB API wrapper for 🦀Rust.
//!
//! Copyright (c) 2023-present Goldy
//! 
//! -------------
//! 
//! # Examples
//! ### How to retrieve a random anime girl holding a programming book in Rust.
//! ```rust
//! use tokio::fs;
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let book = aghpb::random(None).await?;
//! 
//!     println!("Name: {}", book.name);
//!     println!("Category: {}", book.category);
//!     println!("Date added: {}", book.date_added);
//! 
//!     fs::write("./anime_girl.png", book.raw_bytes).await?;
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
//! 

pub mod client;
pub mod book;

pub use book::*;
pub use client::*;

use std::{sync::OnceLock, error::Error};

static _CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> Client {
    let client = _CLIENT.get();

    if let Some(random_client) = client {
        random_client.clone()
    } else {
        let new_client = Client::new(None);
        let _ = _CLIENT.set(new_client.clone());
        new_client
    }
}

/// Asynchronously grabs a random anime girl holding a programming book.
/// 
/// NOTE: Use aghpb::Client for multiple requests. This uses a global client!
/// If you want more customization/speed it maybe preferable to make
/// your own client. 
/// 
/// Uses the ``/v1/random`` endpoint.
pub async fn random(category: Option<String>) -> Result<Book, Box<dyn Error>> {
    get_client().random(category).await
}

/// Asynchronously grabs list of available categories.
/// 
/// NOTE: Use aghpb::Client for multiple requests. This uses a global client!
/// If you want more customization/speed it maybe preferable to make
/// your own client. 
/// 
/// Uses the ``/v1/categories`` endpoint.
pub async fn categories() -> Result<Vec<String>, reqwest::Error> {
    get_client().categories().await
}

/// Allows you to search for anime girls holding programming books.
/// 
/// NOTE: Use aghpb::Client for multiple requests. This uses a global client!
/// If you want more customization/speed it maybe preferable to make
/// your own client. 
/// 
/// Uses the ``/v1/search`` endpoint.
pub async fn search(query: String, category: Option<String>, limit: Option<u8>) -> Result<Vec<BookResult>, reqwest::Error> {
    get_client().search(query, category, limit).await
}