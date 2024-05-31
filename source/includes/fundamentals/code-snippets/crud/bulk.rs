use futures::{ StreamExt, TryStreamExt };
use mongodb::{ bson::doc, bson::Document, Client, Collection, error::Result, options::{ FindOptions, CursorType } };

use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Mushroom {
    name: String,
    color: String,
    edible: bool,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<Document> = client.database("db").collection("mushrooms");

    // begin-sample-data
    let docs = vec! [
        Mushroom { 
            name: "portobello".to_string(), 
            color: "brown".to_string(),
            edible: true
        },
        Mushroom { 
            name: "chanterelle".to_string(), 
            color: "yellow".to_string(),
            edible: true
        },
        Mushroom { 
            name: "fly agaric".to_string(), 
            color: "red".to_string(),
            edible: false
        },
    ];
    // end-sample-data

    let insert_many_result = my_coll.insert_many(docs, None).await?;

    // begin-insert
    let models = vec![
        InsertOneModel::builder()
            .namespace(my_coll.namespace())
            .document(Mushroom { name: "lion's mane".to_string(), color: "white".to_string(), edible: true })
            .build();
        InsertOneModel::builder()
            .namespace(my_coll.namespace())
            .document(Mushroom { name: "angel wing".to_string(), color: "white".to_string(), edible: false })
            .build();
    ];

    let result = client.bulk_write(models).await?;
    println!("Inserted documents: {}", result.inserted_count);
    // end-insert

    // begin-replace
    let models = vec![
        ReplaceOneModel::builder()
            .namespace(my_coll.namespace())
            .filter(doc! { "name": "portobello" })
            .document(Mushroom { name: "cremini".to_string(), color: "brown".to_string(), edible: true })
            .build();
        ReplaceOneModel::builder()
            .namespace(my_coll.namespace())
            .filter(doc! { "name": "oyster" })
            .document(Mushroom { name: "golden oyster".to_string(), color: "yellow".to_string(), edible: true })
            .upsert(true)
            .build();
    ];

    let result = client.bulk_write(models).await?;
    println!("Modified documents: {}", result.modified_count);
    // end-replace

    // begin-update
    let models = vec![
        UpdateOneModel::builder()
            .namespace(my_coll.namespace())
            .filter(doc! { "name": "fly agaric" })
            .update(doc! { "$set": { "name": "fly amanita" } })
            .upsert(true)
            .build();
        UpdateManyModel::builder()
            .namespace(my_coll.namespace())
            .filter(doc! { "color": "yellow" })
            .update(doc! { "$set": { "color": "yellow/orange" } })
            .build();
    ];

    let result = client.bulk_write(models).await?;
    println!("Modified documents: {}", result.modified_count);
    // end-update

    // begin-delete
    let models = vec![
        DeleteOneModel::builder()
            .namespace(my_coll.namespace())
            .filter(doc! { "color": "brown" })
            .build();
        DeleteManyModel::builder()
            .namespace(my_coll.namespace())
            .filter(doc! { "edible": true })
            .build();
    ];

    let result = client.bulk_write(models).await?;
    println!("Deleted documents: {}", result.deleted_count);
    // end-delete

    // begin-options
    let models = vec![
        DeleteOneModel::builder()
            .namespace(my_coll.namespace())
            .filter(doc! { "color": "purple" })
            .build();
        InsertOneModel::builder()
            .namespace(my_coll.namespace())
            .document(Mushroom { name: "reishi".to_string(), color: "red/brown".to_string(), edible: true })
            .build();
    ];

    let result = client.bulk_write(models).ordered(false).await?;
    println!("Inserted documents: {}\nDeleted documents: {}", result.inserted_count, result.deleted_count);
    // end-options

    // begin-mixed-namespaces
    let mushrooms: Collection<Mushroom> = client.database("db").collection("mushrooms");
    let students: Collection<Student> = client.database("people").collection("students");

    let models = vec![
        InsertOneModel::builder()
            .namespace(mushrooms.namespace())
            .document(Mushroom { name: "shiitake".to_string(), color: "brown".to_string(), edible: true })
            .build();
        InsertOneModel::builder()
            .namespace(students.namespace())
            .document(Student { name: "Alex Johnson".to_string(), age: 8 })
            .build();
    ];

    let result = client.bulk_write(models).await?;
    // end-mixed-namespaces

    Ok(())
}