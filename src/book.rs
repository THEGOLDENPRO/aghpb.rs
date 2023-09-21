use bytes::Bytes;
use chrono::{DateTime, FixedOffset};
use image::DynamicImage;

pub struct Book {
    pub name: String,
    pub category: String,
    pub date_added: DateTime<FixedOffset>,
    pub raw_bytes: Bytes,
}

impl Book {
    pub fn to_image(&self) -> DynamicImage {
        image::load_from_memory(&self.raw_bytes).expect("Failed to convert bytes into image.")
    }
}