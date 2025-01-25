use reqwest::blocking::{get, Client};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use url::Url;

/// This struct holds the necessary information for the web crawler.
pub struct WebCrawler {
    client: Client,
    visited_urls: HashSet<String>,
}

impl WebCrawler {
    /// Initializes a new `WebCrawler`.
    pub fn new() -> Self {
        WebCrawler {
            client: Client::new(),
            visited_urls: HashSet::new(),
        }
    }

    /// This function takes a base URL and a file path containing routes,
    /// checks the routes by sending GET requests, and returns the number of valid routes.
    pub fn check_valid_routes(&mut self, base_url: &str, file_path: &str) -> Result<usize, String> {
        let mut num_of_valid_routes = 0;

        // Define the path to the file
        let path = Path::new(file_path);

        // Open the file
        let file = File::open(&path).map_err(|_| "Could not open file")?;

        // Create a buffered reader
        let reader = BufReader::new(file);

        // Read the file line by line
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let mut input_url = base_url.to_string();
                    if input_url.ends_with("/") {
                        input_url.pop();
                    }

                    // Concatenate the base URL with the route
                    let full_url = format!("{}/{}", input_url, line.trim());

                    // Make an HTTP GET request to the concatenated URL
                    match self.get_url(&full_url) {
                        Ok(_) => num_of_valid_routes += 1,
                        Err(_) => continue, // Skip invalid URLs
                    }
                }
                Err(_) => continue, // Skip invalid lines
            }
        }

        Ok(num_of_valid_routes)
    }

    /// This function sends a GET request to the URL and adds it to the visited set if successful.
    fn get_url(&mut self, url: &str) -> Result<(), String> {
        // If the URL has already been visited, skip it
        if self.visited_urls.contains(url) {
            return Ok(());
        }

        match get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    self.visited_urls.insert(url.to_string());
                    Ok(())
                } else {
                    Err(format!("Failed to reach URL: {}", url))
                }
            }
            Err(_) => Err(format!("Failed to send request to URL: {}", url)),
        }
    }
}
