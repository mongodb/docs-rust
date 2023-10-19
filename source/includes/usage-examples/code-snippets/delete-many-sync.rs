use mongodb::{
    bson::{doc, Document},
    sync::{Client, Collection}
};
use serde::{ Deserialize, Serialize };
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    address: Address,
    borough: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
}

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri)?;

    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter =
        doc! { "$and": [
           doc! { "borough": "Manhattan" },
           doc! { "address.street": "Broadway" }
       ]
    };

    let result = my_coll.delete_many(filter, None)?;

    println!("Deleted documents: {}", result.deleted_count);

    Ok(())
}