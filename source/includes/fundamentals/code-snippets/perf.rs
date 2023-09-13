async fn handle_request() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://example.com").await?;
    // Do something with the client
    Ok(())
}

async fn handle_request(client: &Client) -> Result<(), Box<dyn Error>> {
    // Do something with the client
    Ok(())
}
