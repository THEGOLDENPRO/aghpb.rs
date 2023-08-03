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
//! 

pub mod client;
pub mod book;

pub use book::*;
pub use client::*;

use std::sync::OnceLock;

static _CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> Client {
    let client = _CLIENT.get();

    if let Some(random_client) = client {
        random_client.clone()
    } else {
        let new_client = Client::new(None);
        _CLIENT.set(new_client.clone()).expect("Failed to initialize client");
        new_client
    }
}

/// Asynchronously grabs a random anime girl holding a programming book.
/// 
/// NOTE: this uses the global client!
/// If you want more customization/speed it maybe preferable to make
/// your own client.
/// 
/// Uses the ``/v1/random`` endpoint.
pub async fn random(category: Option<&str>) -> Result<Book, reqwest::Error> {
    let client = get_client();
    client.random(category).await
}

/// Asynchronously grabs list of available categories.
/// 
/// NOTE: this uses the global client!
/// If you want more customization/speed it maybe preferable to make
/// your own client.
/// 
/// Uses the ``/v1/categories`` endpoint.
pub async fn categories() -> Result<Vec<String>, reqwest::Error> {
    let client = get_client();
    client.categories().await
}

