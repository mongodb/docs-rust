use std::env;
use mongodb::{
    bson::{ Document, doc },
    sync::{ Client, Collection }
};

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";

    let client = Client::with_uri_str(uri)?;
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! { "name": "Spice Market" };
    let update = doc! { "$set": doc! {"price": "$$$"} };

    let res = my_coll.update_one(filter, update).run()?;
    println!("Updated documents: {}", res.modified_count);

    Ok(())
}
