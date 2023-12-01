use mongodb::{
    bson::{ doc, Document }, 
    options::{ChangeStreamOptions, FullDocumentType},
    Client, Collection
};
use futures_util::{
    StreamExt
};
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Actor {
    name: String,
    starred: Vec,
    oscars: u32,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;
    let my_db = client.database("db");
    let my_coll: Collection<Actor> = my_db.collection("actors");

    // start-docs
    let docs = vec! [
        Actor {
            name: "Emma Stone".to_string(),
            starred: vec! ["Poor Things".to_string(), "La La Land".to_string()],
            oscars: 1,
        },
        Actor {
            name: "Ryan Gosling".to_string(),
            starred: vec! ["Drive".to_string(), "La La Land".to_string()],
            oscars: 0,
        }
    ];
    // end-docs
    
    let insert_many_result = my_coll.insert_many(docs, None).await?;

    // start-open
    let mut change_stream = my_coll.watch(None, None).await?;

    while let Some(event) = change_stream.next().await.transpose()? {
        println!("Operation performed: {}, document: {}", event.operation_type, event.full_document);
    }
    // end-open

    // start-pipeline
    let pipeline = vec![
        doc! { "$match" : doc! { "operationType" : "update" } }
    ];

    let mut update_change_stream = my_coll.watch(pipeline, None).await?;
    while let Some(event) = update_change_stream.next().await.transpose()? {
        println!("Operation performed: {:?}, document: {:?}", event.operation_type, event.full_document);
    }
    // end-pipeline

    // start-options
    let full_doc = Some(FullDocumentType::UpdateLookup);
    let opts = ChangeStreamOptions::builder()
        .full_document(full_doc)
        .build();

    let mut change_stream = my_coll.watch(None, opts).await?;
    while let Some(event) = change_stream.next().await.transpose()? {
        println!("Operation performed: {:?}, document: {:?}", event.operation_type, event.full_document);
    }
    // end-options

    Ok(())
}