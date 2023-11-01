use std::env;

use mongodb::{ bson::doc, sync::{ Client, Collection } };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
}

fn main() -> mongodb::error::Result<()> {
    let uri = match env::var("MONGODB_URI") {
        Ok(val) => val,
        Err(_e) => "No env variable found".to_string(),
    };

    let client = Client::with_uri_str(uri)?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let ct = my_coll.estimated_document_count(None)?;
    println!("Number of documents: {}", ct);

    let ct = my_coll.count_documents(doc! { "name": doc! { "$regex": "Sunset" } }, None)?;
    println!("Number of matching documents: {}", ct);

    Ok(())
}
