use std::env;
use mongodb::{ bson::doc, error::Result, Client };

#[tokio::main]
async fn main() -> Result<()> {
    // begin-subscriber
    tracing_subscriber::fmt::init();
    // end-subscriber

    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    // start-operation
    let my_coll = client.database("db").collection("test_coll");
    my_coll.insert_one(doc! { "x" : 1 }, None).await?;
    // end-operation

    Ok(())
}
