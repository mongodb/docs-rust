use mongodb::{ 
    bson::{Document, doc},
    Client,
    Collection 
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";

    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Document> = client.database("sample_guides").collection("planets");

    let filter = doc! { "orderFromSun": doc!{ "$lte": 3 } };

    let result = my_coll.delete_many(filter, None).await?;
    println!("Deleted documents: {}", result.deleted_count);

    Ok(())
}
