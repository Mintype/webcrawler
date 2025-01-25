use reqwest::blocking::get;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {

    let mut num_of_valid_routes = 0;

    // Create a new String to hold the user input
    let mut input_url = String::new();

    // Print a message to the user
    println!("Enter a URL:");

    // Read the user input
    std::io::stdin()
        .read_line(&mut input_url)
        .expect("Failed to read URL");

    // Trim the input URL
    input_url = input_url.trim().to_string();


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
                if input_url.ends_with("/") {
                    input_url.pop();
                }
                
                // Concatenate the input URL with the route without moving input_url
                let full_url = format!("{}/{}", input_url, line.trim());

                // Make an HTTP GET request to the concatenated URL
                match get(&full_url) {
                    Ok(response) => {
                        println!("URL: {} - Status Code: {}", full_url, response.status());
                        if response.status().is_success() {
                            num_of_valid_routes += 1;
                        }
                    }
                    Err(_) => {
                        println!("URL: {} - Failed to get a response", full_url);
                    }
                }
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }
    println!("Number of valid routes: {}", num_of_valid_routes);
}