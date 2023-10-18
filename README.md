# `dmzj`

[![Cargo](https://img.shields.io/crates/v/dmzj.svg)](https://crates.io/crates/dmzj) [![Documentation](https://docs.rs/dmzj/badge.svg)](https://docs.rs/dmzj)

An async dmzj client for Rust.

Mostly taken from [tachiyomi extensions](https://github.com/tachiyomiorg/tachiyomi-extensions/tree/master/src/zh/dmzj) & [flutter_dmzj](https://github.com/xiaoyaocz/flutter_dmzj).

## Example

```rust
use dmzj::Api;

#[tokio::main]
async fn main() {
    let api = Api::new();

    let popular_manga = api.fetch_latest_updates_manga(0).await.unwrap();
    let first = &popular_manga[0];
    let first_id = first.id;

    println!("manga id = {:?}", first_id);

    let response = api.fetch_manga_details(first_id).await.unwrap();

    let description = &response.data.description;
    let title = &response.data.title;
    let cover = &response.data.cover;
    let authors = &response.data.authors;

    println!("title = {}", title);
    println!("description = {}", description);
    println!("cover = {}", cover);
    println!("authors = {:?}", authors);
}
```