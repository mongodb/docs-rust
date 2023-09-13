#[tokio::test]
async fn test_list_dbs() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://example.com").await?;
    CLIENT.list_database_names(None, None).await?;
    Ok(())
}
