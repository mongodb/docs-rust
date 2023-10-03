use tokio::runtime::Runtime;
use once_cell::sync::Lazy;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        Client::with_uri_str("<connection string>").await.unwrap()
    })
});

// This will inconsistently fail.
#[tokio::test]
async fn test_list_dbs() -> Result<(), Box<dyn Error>> {
    CLIENT.list_database_names(None, None).await?;
    Ok(())
}
