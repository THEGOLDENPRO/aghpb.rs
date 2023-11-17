use std::{collections::HashMap, error::Error};

use bytes::Bytes;
use chrono::{DateTime, FixedOffset};
use image::DynamicImage;
use reqwest::header::HeaderMap;

use crate::get_id;

pub struct BookData {
    pub name: String,
    pub category: String,
    pub date_added: DateTime<FixedOffset>,
    pub search_id: String,
    pub commit_url: String,
    pub commit_author: String,
}

impl BookData {
    /// Creates a book from a json dictionary.
    pub fn from_json(book_dict: HashMap<String, String>) -> BookData {
        let name = book_dict.get("name").expect(
            "Failed acquiring book name from json!"
        ).to_owned();

        let category = book_dict.get("category").expect(
            "Failed acquiring book category from json!"
        ).to_owned();

        let date_added = DateTime::parse_from_str(book_dict.get("date_added").expect(
            "Failed acquiring book date added header!"
        ), "%Y-%m-%d %H:%M:%S%z").expect(
            "Failed to convert book's date added header to date time object."
        );

        let search_id = book_dict.get("search_id").expect(
            "Failed acquiring book search id header!"
        ).to_owned();

        let commit_url = book_dict.get("commit_url").expect(
            "Failed acquiring book commit url header!"
        ).to_owned();

        let commit_author = book_dict.get("commit_author").expect(
            "Failed acquiring book commit author header!"
        ).to_owned();

        Self {
            name,
            category,
            date_added,
            search_id,
            commit_url,
            commit_author
        }
    }

    // Get's the book from the api.
    pub async fn get_book(&self) -> Result<Book, Box<dyn Error>> {
        get_id(String::from(&self.search_id)).await
    }
}

pub struct Book {
    pub details: BookData,
    pub raw_bytes: Bytes,
}

impl Book {
    /// Creates a book from a response's headers and bytes.
    pub fn from_response(headers: HeaderMap, bytes: Bytes) -> Book {
        let name = headers.get("book-name").expect(
            "Failed acquiring book name header!"
        ).to_str().expect("Failed converting book name to string.").to_owned();
    
        let category = headers.get("book-category").expect(
            "Failed acquiring book category header!"
        ).to_str().expect("Failed converting book category to string.").to_owned();
    
        let date_added = DateTime::parse_from_str(headers.get("book-date-added").expect(
            "Failed acquiring book date added header!"
        ).to_str().expect("Failed converting book date time to string."), "%Y-%m-%d %H:%M:%S%z").expect(
            "Failed to convert book's date added header to date time object."
        );
    
        let search_id = headers.get("book-search-id").expect(
            "Failed acquiring book search id header!"
        ).to_str().expect("Failed converting book search id to string.").to_owned();
    
        let commit_url = headers.get("book-commit-url").expect(
            "Failed acquiring book commit url header!"
        ).to_str().expect("Failed converting book commit url to string.").to_owned();
    
        let commit_author = headers.get("book-commit-author").expect(
            "Failed acquiring book commit author header!"
        ).to_str().expect("Failed converting book commit author to string.").to_owned();

        Self {
            details: BookData { 
                name, 
                category, 
                date_added, 
                search_id, 
                commit_url, 
                commit_author 
            },
            raw_bytes: bytes
        }
    }

    pub fn to_image(&self) -> DynamicImage {
        image::load_from_memory(&self.raw_bytes).expect("Failed to convert bytes into dynamic image object.")
    }
}