// Import the reqwest crate
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Specify the URL of the API endpoint
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // Send an HTTP GET request to the specified URL
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        // Parse the JSON response body
        let body = response.text().await?;

        // Print the response body
        println!("Response body: {}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
