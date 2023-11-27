use mongodb::{
    bson::{ Bson, doc, oid::ObjectId }, 
    Client
};
use futures_util::{
    io::AsyncWriteExt,
    AsyncReadExt,
    TryStreamExt
};
use std::{
    fs,
    str::FromStr,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri: &str = "<connection string>";
    let client = Client::with_uri_str(uri).await?;
    let my_db = client.database("db");

    // start-create
    let bucket = my_db.gridfs_bucket(None);
    // end-create

    // start-create-opts
    let opts = GridFsBucketOptions::builder()
        .bucket_name("my_bucket".to_string())
        .build();
    let bucket = my_db.gridfs_bucket(opts);
    // end-create-opts

    // start-upload
    let filename = "example.txt";
    let file_bytes = fs::read(filename).await?;
    let mut upload_stream = bucket.open_upload_stream("example", None);
    upload_stream.write_all(&file_bytes[..]).await?;
    println!("Document uploaded with ID: {}", upload_stream.id());
    upload_stream.close().await?;
    // end-upload

    // start-retrieve 
    let filter = doc! {};
    let mut cursor = bucket.find(filter, None).await?;
    while let Some(result) = cursor.try_next().await? {
        println!("File length: {}\n", result.length);
    };
    // end-retrieve

    // start-download
    let id = ObjectId::from_str("3289").expect("Could not convert to ObjectId");
    let mut buf = Vec::new();
    let mut download_stream = bucket.open_download_stream(Bson::ObjectId(id)).await?;
    let result = download_stream.read_to_end(&mut buf).await?;
    println!("{:?}", result);
    // end-download

    // start-rename
    let id = ObjectId::from_str("3289").expect("Could not convert to ObjectId");
    let new_name = "new_file_name";
    bucket.rename(Bson::ObjectId(id), &new_name).await?;
    // end-rename

    // start-delete-file
    let id = ObjectId::from_str("3289").expect("Could not convert to ObjectId");
    bucket.delete(Bson::ObjectId(id)).await?;
    // end-delete-file

    // start-delete-bucket
    bucket.drop().await?;
    // end-delete-bucket

    Ok(())
}