use reqwest::blocking::get;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // Create a new String to hold the user input
    let mut inputURL = String::new();

    // Print a message to the user
    println!("Enter a URL:");

    // Read the user input
    std::io::stdin()
        .read_line(&mut inputURL)
        .expect("Failed to read URL");

    // Trim the input URL
    inputURL = inputURL.trim().to_string();

    // Make an HTTP GET request to the URL
    match get(&inputURL) {
        Ok(response) => {
            if response.status().is_success() {
                // Print the response text
                match response.text() {
                    Ok(text) => println!("Response: {}", text),
                    Err(e) => println!("Failed to read response text: {}", e),
                }
            } else {
                println!("Request failed with status: {}", response.status());
            }
        }
        Err(e) => println!("Failed to make request: {}", e),
    }

    // Define the path to the file
    let path = Path::new("src/routes.txt");

    // Open the file
    let file = File::open(&path).expect("Could not open file");

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Concatenate the input URL with the route without moving input_url
                let full_url = format!("{}/{}", inputURL, line.trim());

                // // Print the concatenated URL
                println!("Full URL: {}", full_url);

                // Make an HTTP GET request to the concatenated URL
                match get(&full_url) {
                    Ok(response) => {
                        println!("URL: {} - Status Code: {}", full_url, response.status());
                    }
                    Err(_) => {
                        println!("URL: {} - Failed to get a response", full_url);
                    }
                }
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }
}