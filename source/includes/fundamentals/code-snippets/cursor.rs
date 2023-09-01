use bson::Document;
use futures::{ StreamExt, TryStreamExt };
use mongodb::{ bson::doc, Client, error::Result, options::{ FindOptions, CursorType } };

#[tokio::main]
async fn main() -> Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    let my_coll = client.database("db").collection::<Document>("fruits");

    // start-indiv-builtin
    let mut cursor = my_coll.find(doc! { "color": "red" }, None).await?;
    while cursor.advance().await? {
        println!("{:?}", cursor.deserialize_current()?);
    }
    // end-indiv-builtin

    // start-indiv-stream
    let mut cursor = my_coll.find(doc! { "color": "red" }, None).await?;
    println!("\nOutput from next() iteration:");
    while let Some(doc) = cursor.next().await {
        println!("{}", doc?);
    }

    let mut cursor = my_coll.find(doc! { "color": "yellow" }, None).await?;
    println!("\nOutput from try_next() iteration:");
    while let Some(doc) = cursor.try_next().await? {
        println!("{}", doc);
    }
    // end-indiv-stream

    // start-array
    let cursor = my_coll.find(doc! { "color": "red" }, None).await?;
    println!("\nOutput from collect():");
    let v: Vec<Result<Document>> = cursor.collect().await;
    println!("{:?}", v);

    let cursor = my_coll.find(doc! { "color": "yellow" }, None).await?;
    println!("\nOutput from try_collect():");
    let v: Vec<Document> = cursor.try_collect().await?;
    println!("{:?}", v);
    // end-array

    // start-options
    let _opts: FindOptions = FindOptions::builder()
        .batch_size(5)
        .cursor_type(CursorType::Tailable)
        .no_cursor_timeout(true)
        .build();
    // end-options

    Ok(())
}
