use mongodb::{
    bson::{doc, Document},
    sync::{Client, Collection},
    options::FindOptions
};
use serde::{ Deserialize, Serialize };
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
}

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri)?;

    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let opts = FindOptions::builder()
        .sort(doc! { "name": 1 })
        .build();

    let mut cursor = my_coll.find(
        doc! { "cuisine": "French" },
        opts
    )?;
    
    for result in cursor {
        println!("{:?}", result?);
      }

    Ok(())
}
