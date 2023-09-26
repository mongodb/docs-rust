use mongodb::{ bson::doc, Client, Collection };
use serde::{ Deserialize, Serialize };
use std::env;

// begin-veg-struct
#[derive(Serialize, Deserialize)]
struct Vegetable {
    name: String,
    category: String,
    protein_content: f32,
    other_names: Vec<String>,
}
// end-veg-struct

#[derive(Serialize, Deserialize)]
struct Square {
    side_length: f32,
}

#[derive(Serialize, Deserialize)]
struct Circle {
    radius: f32,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri: &str = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    // begin-access-coll
    let my_coll: Collection<Vegetable> = client.database("db").collection("vegetables");
    // end-access-coll

    // begin-insert-veg
    let calabash = Vegetable {
        name: "calabash".to_string(),
        category: "gourd".to_string(),
        protein_content: 0.6,
        other_names: vec!["bottle gourd".to_string(), "lauki".to_string()],
    };

    my_coll.insert_one(calabash, None).await?;
    // end-insert-veg

    // begin-multiple-types
    let shapes_coll: Collection<Square> = client.database("db").collection("shapes");
    // ... perform some operations with Square

    let shapes_coll: Collection<Circle> = shapes_coll.clone_with_type();
    // ... perform some operations with Circle
    // end-multiple-types

    Ok(())
}
