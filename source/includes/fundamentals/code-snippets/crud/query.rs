use bson::Document;
use futures::TryStreamExt;
use mongodb::{ bson::doc, Client, Collection, options::{ FindOptions } };

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Document> = client.database("db").collection("fruits");
    // Sample documents
    let docs = vec! [
       doc! { "_id": 1, "name": "orange", "quantity": 7, "price": 5 },
       doc! { "_id": 2, "name": "apple", "quantity": 4, "price": 2, "description": "a red or green fruit" },
       doc! { "_id": 3, "name": "banana", "quantity": 36, "price": 4 },
       doc! { "_id": 4, "name": "pear", "quantity": 28, "price": 6 }
    ];
    // Inserts sample documents into the collection
    my_coll.insert_many(docs, None).await?;

    //begin-literal
    let query = doc! { "name": "pear" };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    } 
    //end-literal

    //begin-comparison
    // $gt means "greater than"
    let query = doc! { "quantity": doc! { "$gt": 5 } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-comparison

    //begin-literal
    let query = doc! { "name": "pear" };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    } 
    //end-literal
    println!("");
    //begin-comparison
    // $gt means "greater than"
    let query = doc! { "quantity": doc! { "$gt": 5 } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-comparison
    println!("");
    //begin-logical
    let query = doc! { "$and": vec! [
           doc! { "qty": doc! { "$gt": 10 } },
           doc! {"price" : doc! {"$lt": 5 } }
       ]
    };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-logical
    println!("");
    //begin-element
    let query = doc! { "description": doc! { "$exists": true } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
      let doc: Document = bson::from_document(result)?;
      println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-element
    println!("");
    //begin-evaluation
    // $mod means "modulo" and returns the remainder after division
    let query = doc! { "quantity": doc! { "$mod": [ 3, 0 ] } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
       let doc: Document = bson::from_document(result)?;
       println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-evaluation
    println!("");
    //begin-bitwise
    let query = doc! { "price": doc! { "$bitsAllSet": 6 } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
        let doc: Document = bson::from_document(result)?;
        println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-bitwise
    println!("");
    //begin-array
    let query = doc! { "vendors": doc! { "$all": [ "A", "C" ] } };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(result) = cursor.try_next().await? {
       let doc: Document = bson::from_document(result)?;
       println!("{}", serde_json::to_string_pretty(&doc).unwrap());
    }
    //end-array

    Ok(())
}