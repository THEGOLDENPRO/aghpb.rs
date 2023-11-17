<div align="center">

  # 🦀 aghpb.rs 📚
  <sub>Rust API wrapper for the anime girls holding programming books [API](https://api.devgoldy.xyz/aghpb/v1/docs)</sub>

  [![Crates.io](https://img.shields.io/crates/v/aghpb?style=flat)](https://crates.io/crates/aghpb)
  [![docs.rs](https://img.shields.io/docsrs/aghpb?style=flat)](https://docs.rs/aghpb)

</div>

<div align="center">

  <img src="./assets/book_1.png" width="600px">

</div>

<br>

> [!Note]
> 
> This is part of my [aghpb api](https://github.com/THEGOLDENPRO/aghpb_api) wrapper challenge where I attempt to write an api wrapper in every language possible. So yes expect spaghetti code as it will be my first time writing in these languages. Although I'm 100% open to improvements and corrections so feel free to contribute anything.
> **[Other languages I've done](https://github.com/THEGOLDENPRO/aghpb_api#-api-wrappers)**

## Install
```rust
cargo add aghpb
```
More install instructions at [crates.io](https://crates.io/crates/aghpb).

## Examples
This is how you may retrieve a random anime girls holding programming books:
```rust
use tokio::fs;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let book = aghpb::random(None).await?;

    println!("Name: {}", book.name);
    println!("Category: {}", book.category);
    println!("Date added: {}", book.date_added);

    fs::write("./anime_girl.png", book.raw_bytes).await?;

    Ok(())
}
```
You can also retrieve specific categories of anime girls holding programming books like so:
```rust
let book = aghpb::random(Some("rust".into())).await?;
```

<br>

This is how you may retrieve a list of available categories:
```rust
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let categories = aghpb::categories().await?;

    for category in categories {
        println!("{}", category);
    }

    Ok(())
}
```

<br>

> [!INFO]
> NEW in v1.4!

How to search for an anime girl holding a programming book.
```rust
use std::error::Error;

use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let books = aghpb::search("tohru".into(), None, None).await?;

    let book_data = &books[0]; // I'm selecting the first book just for this example.

    println!("Name: {}", book_data.name);
    println!("Category: {}", book_data.category);
    println!("Commit Author: {}", book_data.commit_author);
    println!("Commit URL: {}", book_data.commit_url);
    println!("Date Added: {}", book_data.date_added);
    println!("Search ID: {}", book_data.search_id);

    let book = book_data.get_book().await?;

    fs::write("./anime_girl.png", book.raw_bytes).await?;

    Ok(())
}
```

Made using my API at 👉 https://api.devgoldy.xyz/aghpb/v1/
