use mongodb::{
    bson::{doc, oid::ObjectID},
    sync::{Client, Collection}
};
use std::str::FromStr;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    _id: ObjectId,
    name: String,
}

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri)?;

    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let id = ObjectId::from_str("5eb3d668b31de5d588f42bfc").expect("Could not convert to ObjectID");
    let filter = doc! { "_id": id };

    let result = my_coll.delete_one(filter, None)?;

    println!("Deleted documents: {}", result.deleted_count);
        
    Ok(())
}