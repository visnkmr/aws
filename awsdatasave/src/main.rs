
use std::fs::File;
use std::io::Write;
use chrono::Local;
use reqwest::Error;

async fn fetch_and_save(url: &str, file_path: &str) -> Result<(), Error> {
    // Send a GET request to the specified URL
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Get the response body as bytes
        let content = response.bytes().await?;

        // Create a file and write the response content to it
        let mut file = File::create(file_path).unwrap();
        file.write_all(&content);
        println!("File saved to {}", file_path);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // URL of the endpoint
    let url = "http://aws.imd.gov.in:8091/AWS/temp.php?a=60&b=ALL_STATE";
    
    // Get the current date and time
    let now = Local::now();
    // Format the date and time as a string
    let formatted_date_time = now.format("%Y%m%d_%H").to_string();

    // Path to save the file, including the date and time
    let file_path = format!("aws_{}.txt", formatted_date_time);

    // Fetch the response and save it to a file
    fetch_and_save(url, &file_path).await
}
