use bson::Document;
use mongodb::{ bson::doc, Client, Collection };
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    // begin-update
    let my_coll: Collection<Document> = client.database("db").collection("employees");
    let update_doc =
        doc!(
        "$set": doc!{ "department": "Business Operations", 
                      "role": "Analytics Specialist" },
        "$inc": doc!{ "bonus": 500 }
    );

    let res = my_coll.update_many(doc! { "department": "Marketing" }, update_doc, None).await?;
    println!("Modified {} document(s)", res.modified_count);
    // end-update

    Ok(())
}
