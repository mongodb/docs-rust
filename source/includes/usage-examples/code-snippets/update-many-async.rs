use std::env;
use mongodb::{ bson::doc, Client, Collection };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    borough: String,
    address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";

    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! { "address.street": "Sullivan Street", "borough": "Manhattan" };
    let update = doc! { "$set": doc! { "near_me": true } };

    let res = my_coll.update_many(filter, update, None).await?;
    println!(
        "Matched documents: {}\nUpdated documents: {}",
        res.matched_count,
        res.modified_count
    );

    Ok(())
}
