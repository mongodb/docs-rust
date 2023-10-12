use futures::TryStreamExt;
use mongodb::{ bson::doc, Client, Collection, options::FindOptions };
use serde::{ Deserialize, Serialize };
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    address: Address,
    cuisine: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    coord: Vec<f64>,
    zipcode: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = match env::var("MONGO_URI") {
        Ok(val) => val,
        Err(_e) => "No env variable found".to_string(),
    };
    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let mut cursor = my_coll.find(
        doc! { "cuisine": "French" },
        None
    ).await?;

    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }

    Ok(())
}



