use bson::Document;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{ bson::doc, Client, Collection, options::{ FindOptions } };

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Document> = client.database("db").collection("");
    
    //begin cursor def
    let cursor = my_coll.find(None, None).await?;
    //end cursor def

    
    //begin try next
    let cursor = my_coll.find(None, None).await?;
    while let Some(result) = cursor.try_next().await? {
       let doc: Document = bson::from_document(result)?;
       println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end try next

    //begin-cursor-next
    let cursor = my_coll.find(None, None).await?;
    while let Some(result) = cursor.next().await {
       let doc: Document = bson::from_document(result)?;
       println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    } 
    //end-cursor-next

    //begin cursor collect
    let cursor = my_coll.find(None, None).await?;
    // Stream uses populate vector containing all documents in the cursor
    let v: Vec<_> = cursor.collect().await;
    // Print data in vector
    for i in v.iter(){
        let doc: Document = bson::from_document(i.clone().ok().unwrap())?;
        println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end cursor collect

    //begin try collect
    let cursor = my_coll.find(None, None).await?;
    // TryStream uses try_collect() and collects into a Result<Vec<T>>
    let v: Vec<_> = cursor.try_collect().await?;
    // Print data in vector
    for i in v.iter(){
        let doc: Document = bson::from_document(i.clone())?;
        println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end try collect