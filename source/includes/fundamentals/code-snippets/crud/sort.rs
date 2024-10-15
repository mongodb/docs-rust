use std::env;
use mongodb::{ bson::doc, bson::Document, Client, Collection, options::{ FindOptions, ClientOptions } };
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

// start-book-struct
#[derive(Debug, Serialize, Deserialize)]
struct Book {
    #[serde(rename = "_id")]
    id: i32,
    name: String,
    author: String,
    length: i32,
}
// end-book-struct

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
// start-sample-data
    let uri = "connection string";
    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Book> = client.database("db").collection("books");

    let books = vec![
        Book {
            id: 1,
            name: "The Brothers Karamazov".to_string(),
            author: "Dostoyevsky".to_string(),
            length: 824,
        },
        Book {
            id: 2,
            name: "Les Misérables".to_string(),
            author: "Hugo".to_string(),
            length: 1462,
        },
        Book {
            id: 3,
            name: "Atlas Shrugged".to_string(),
            author: "Rand".to_string(),
            length: 1088,
        },
        Book {
            id: 4,
            name: "Infinite Jest".to_string(),
            author: "Wallace".to_string(),
            length: 1104,
        },
        Book {
            id: 5,
            name: "Cryptonomicon".to_string(),
            author: "Stephenson".to_string(),
            length: 918,
        },
        Book {
            id: 6,
            name: "A Dance with Dragons".to_string(),
            author: "Martin".to_string(),
            length: 1104,
        },
    ];

    my_coll.insert_many(books).await?;
// end-sample-data

// start-sort-query
let filter = doc! {};

let find_options = FindOptions::builder()
    .sort(doc! { "author": 1 })  // 1 for ascending order, -1 for descending order
    .build();

let mut cursor = my_coll.find(Some(filter), Some(find_options)).await?;

while let Some(result) = cursor.next().await {
    match result {
        Ok(document) => println!("{:?}", document),
        Err(e) => return Err(e.into()),
    }
}
// end-sort-query

// start-sort-aggregation
let pipeline = vec![
    docs! { "$match": {} },
    doc! { "$sort": { "author": 1 } } // 1 for ascending order, -1 for descending order
];

let mut cursor = my_coll.aggregate(pipeline, None).await?;

for result in cursor {
    match result {
        Ok(document) => println!("{:?}", document),
        Err(e) => eprintln!("Error: {}", e),
    }
}
// end-sort-aggregation

// start-ascending-sort
let filter = doc! {};

let find_options = mongodb::options::FindOptions::builder()
.sort(doc! { "name": 1 })
.build();

let mut cursor = my_coll.find(Some(filter), Some(find_options)).await?;

while let Some(result) = cursor.next().await {
    match result {
        Ok(document) => println!("{:?}", document),
        Err(e) => return Err(e.into()),
    }
}
// end-ascending-sort

// start-descending-sort
let filter = doc! {};

let find_options = FindOptions::builder()
.sort(doc! { "name": -1 })
.build();

let mut cursor = my_coll.find(Some(filter), Some(find_options)).await?;

while let Some(result) = cursor.next().await {
    match result {
        Ok(document) => println!("{:?}", document),
        Err(e) => return Err(e.into()),
    }
}
// end-descending-sort

    Ok(())
}