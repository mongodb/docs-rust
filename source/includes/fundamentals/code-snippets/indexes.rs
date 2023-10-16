use bson::{ Document, doc };
use mongodb::{
    Client,
    Collection,
    IndexModel,
    options::{ ClientOptions, ClusteredIndex, CreateCollectionOptions, IndexOptions },
};
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<Document> = client.database("<db>").collection("<coll>");

    // begin-single-field
    let index = IndexModel::builder()
        .keys(doc! { "city": 1 })
        .build();

    let idx = my_coll.create_index(index, None).await?;
    println!("Created index:\n{}", idx.index_name);
    // end-single-field

    // begin-compound
    let index = IndexModel::builder()
        .keys(doc! { "city": 1, "pop": -1 })
        .build();

    let idx = my_coll.create_index(index, None).await?;
    println!("Created index:\n{}", idx.index_name);
    // end-compound

    // begin-multikey
    let index = IndexModel::builder()
        .keys(doc! { "tags": 1 })
        .build();

    let idx = my_coll.create_index(index, None).await?;
    println!("Created index:\n{}", idx.index_name);
    // end-multikey

    let db = client.database("db");
    // begin-clustered
    let cl_idx = ClusteredIndex::default();
    let opts = CreateCollectionOptions::builder()
        .clustered_index(cl_idx)
        .build();

    db.create_collection("items", opts).await?;
    // end-clustered

    // begin-text
    let idx_opts = IndexOptions::builder()
        .default_language("spanish".to_string())
        .build();

    let index = IndexModel::builder()
        .keys(doc! { "body": 1 })
        .options(idx_opts)
        .build();

    let idx = my_coll.create_index(index, None).await?;
    println!("Created index:\n{}", idx.index_name);
    // end-text

    // begin-geo
    let index = IndexModel::builder()
        .keys(doc! { "location": "2dsphere" })
        .build();

    let idx = my_coll.create_index(index, None).await?;
    println!("Created index:\n{}", idx.index_name);
    // end-geo

    // begin-unique
    let opts = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(doc! { "name": -1 })
        .options(opts)
        .build();

    let idx = my_coll.create_index(index, None).await?;
    println!("Created index:\n{}", idx.index_name);
    // end-unique

    // begin-drop
    my_coll.drop_index("city_1".to_string(), None).await?;
    // end-drop

    Ok(())
}
