use mongodb::{
    bson::{doc, Document},
    Client, Collection,
    options::{InsertOneModel, ReplaceOneModel, UpdateOneModel, UpdateManyModel, DeleteOneModel, DeleteManyModel, WriteModel},
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    let mushrooms: Collection<Document> = client.database("db").collection("mushrooms");

    mushrooms.drop().await?;

    // begin-sample-data
    let docs = vec![
        doc! {"name" : "portobello", "color" : "brown", "edible" : true },
        doc! {"name" : "chanterelle", "color" : "yellow", "edible" : true },
        doc! {"name" : "oyster", "color" : "white", "edible" : true },
        doc! {"name" : "fly agaric", "color" : "red", "edible" : false },
    ];
    // end-sample-data

    let _insert_many_result = mushrooms.insert_many(docs).await?;

    // begin-insert
    let mushrooms: Collection<Document> = client.database("db").collection("mushrooms");

    let models = vec![
        InsertOneModel::builder()
            .namespace(mushrooms.namespace())
            .document(doc! { "name": "lion's mane", "color": "white", "edible": true })
            .build(),
        InsertOneModel::builder()
            .namespace(mushrooms.namespace())
            .document(doc! { "name": "angel wing", "color": "white", "edible": false })
            .build(),
    ];

    let result = client.bulk_write(models).await?;
    println!("Inserted documents: {}", result.inserted_count);
    // end-insert

    // begin-replace
    let mushrooms: Collection<Document> = client.database("db").collection("mushrooms");

    let models = vec![
        ReplaceOneModel::builder()
            .namespace(mushrooms.namespace())
            .filter(doc! { "name": "portobello" })
            .replacement(doc! { "name": "cremini", "color": "brown", "edible": true })
            .build(),
        ReplaceOneModel::builder()
            .namespace(mushrooms.namespace())
            .filter(doc! { "name": "oyster" })
            .replacement(doc! { "name": "golden oyster", "color": "yellow", "edible": true })
            .upsert(true)
            .build(),
    ];

    let result = client.bulk_write(models).await?;
    println!("Modified documents: {}", result.modified_count);
    // end-replace

    // begin-update
    let mushrooms: Collection<Document> = client.database("db").collection("mushrooms");

    let models = vec![
        WriteModel::UpdateOne(UpdateOneModel::builder()
            .namespace(mushrooms.namespace())
            .filter(doc! { "name": "fly agaric" })
            .update(doc! { "$set": { "name": "fly amanita" } })
            .upsert(true)
            .build()),
        WriteModel::UpdateMany(UpdateManyModel::builder()
            .namespace(mushrooms.namespace())
            .filter(doc! { "color": "yellow" })
            .update(doc! { "$set": { "color": "yellow/orange" } })
            .build()),
    ];

    let result = client.bulk_write(models).await?;
    println!("Modified documents: {}", result.modified_count);
    // end-update

    // begin-delete
    let mushrooms: Collection<Document> = client.database("db").collection("mushrooms");

    let models = vec![
        WriteModel::DeleteOne(DeleteOneModel::builder()
            .namespace(mushrooms.namespace())
            .filter(doc! { "color": "red" })
            .build()),
        WriteModel::DeleteMany(DeleteManyModel::builder()
            .namespace(mushrooms.namespace())
            .filter(doc! { "edible": true })
            .build()),
    ];

    let result = client.bulk_write(models).await?;
    println!("Deleted documents: {}", result.deleted_count);
    // end-delete

    // begin-options
    let mushrooms: Collection<Document> = client.database("db").collection("mushrooms");

    let models = vec![
        WriteModel::DeleteOne(DeleteOneModel::builder()
            .namespace(mushrooms.namespace())
            .filter(doc! { "color": "purple" })
            .build()),
        WriteModel::InsertOne(InsertOneModel::builder()
            .namespace(mushrooms.namespace())
            .document(doc! { "name": "reishi", "color": "red/brown", "edible": true })
            .build()),
    ];

    let result = client.bulk_write(models).ordered(false).await?;
    println!(
        "Inserted documents: {}\nDeleted documents: {}",
        result.inserted_count, result.deleted_count
    );
    // end-options

    // begin-mixed-namespaces
    let mushrooms: Collection<Document> = client.database("db").collection("mushrooms");
    let students: Collection<Document> = client.database("people").collection("students");

    let models = vec![
        InsertOneModel::builder()
            .namespace(mushrooms.namespace())
            .document(doc! { "name": "shiitake", "color": "brown", "edible": true })
            .build(),
        InsertOneModel::builder()
            .namespace(students.namespace())
            .document(doc! { "name": "Alex Johnson", "age": 8 })
            .build(),
    ];

    let result = client.bulk_write(models).await?;
    println!("Inserted documents: {}", result.inserted_count);
    // end-mixed-namespaces

    Ok(())
}
