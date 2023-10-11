use mongodb::{
    bson::{doc, Document},
    sync::{Client, Collection}
};

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    
    let client = Client::with_uri_str(uri)?;
    let my_coll: Collection<Document> = client.database("sample_guides").collection("planets");

    let filter = doc! { "orderFromSun": doc!{ "$lte": 3 } };

    let result = my_coll.delete_many(filter, None)?;
    println!("Deleted documents: {}", result.deleted_count);

    Ok(())
}
