use bson::{ Document };
use mongodb::{ bson::doc, options::{ CollectionOptions, WriteConcern }, Client, Collection };
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    let db: mongodb::Database = client.database("test_db");

    // begin-document-validation
    let validator =
        doc! {
            "$jsonSchema": doc! {
               "bsonType": "object",
               "title": "Answer Value Validation",
               "properties": doc! {
                  "answer": doc! {
                     "enum": vec! [ "inaccurate", "accurate" ],
                  }
               }
            }
        };
    let validation_opts = CreateCollectionOptions::builder()
        .validator(validator)
        .validation_action(Some(ValidationAction::Error))
        .validation_level(Some(ValidationLevel::Moderate))
        .build();

    db.create_collection("survey_answers", validation_opts).await?;
    // end-document-validation

    Ok(())
}
