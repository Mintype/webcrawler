# Web Crawler Library

This Rust library is a simple web crawler that checks the validity of routes on a given website. It reads a list of routes from a file, constructs full URLs by appending the routes to a base URL, and sends HTTP GET requests to check whether the routes are valid.

## Features

- Reads a list of routes from a text file.
- Constructs URLs by combining the base URL and the routes.
- Makes HTTP GET requests to check if the routes are valid.
- Tracks visited URLs to avoid re-crawling.
- Returns the number of valid routes.

## Installation

### 1. Add `webcrawler` to Your `Cargo.toml`

If you're using this library in your own Rust project, add the following to your `Cargo.toml` under `[dependencies]`:

```toml
[dependencies]
webcrawler = { version = "0.1.1" }
reqwest = { version = "0.11", features = ["blocking"] }
futures-io = "0.3.30"
url = "2.2"
```

## Usage
### 1. Import the Web Crawler
In your main.rs or any other Rust file, import the web crawler library:
```rust
use webcrawler::WebCrawler;
```
### 1. Example usage
Here is an example of how to use the `WebCrawler` to check the validity of routes:
```rust
use webcrawler::WebCrawler;

fn main() {
    let base_url = "http://example.com";  // Replace with your base URL
    let file_path = "src/routes.txt";     // Replace with the path to your routes file

    let mut crawler = WebCrawler::new();

    match crawler.check_valid_routes(base_url, file_path) {
        Ok(valid_routes) => {
            println!("Number of valid routes: {}", valid_routes);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions
Contributions are welcome! If you have any improvements, bug fixes, or feature suggestions, feel free to open an issue or submit a pull request.

## Crates.io
You can find this crate and the latest version on [crates.io](https://crates.io/crates/webcrawler).
