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

<<<<<<< HEAD
    //begin-literal
=======
    //begin-literal-query
>>>>>>> 5d33bbf (post review)
    let query = doc! { "name": "pear" };
    let mut cursor = my_coll.find(query, None).await?;
    while let Some(Document) = cursor.try_next().await? {
       println!("{}", Document);
<<<<<<< HEAD
    } 
    //end-literal
=======
    }
    //end-literal-query
>>>>>>> 5d33bbf (post review)

    //begin-comparison
    // $gt means "greater than"
    let query = doc! { "quantity": doc! { $"gt": 5 } };
<<<<<<< HEAD
    let mut cursor = my_coll.find(query, None).await?;
=======
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = my_coll.find(query, find_options).await?;
>>>>>>> 5d33bbf (post review)
    while let Some(Document) = cursor.try_next().await? {
       println!("{}", Document);
    }
    //end-comparison

    //begin-logical
<<<<<<< HEAD
    let query = doc! { "$and": vec! [
           doc! { "qty": doc! { "$gt": 5 } },
           doc! {"price" : doc! {"$lt": 5 } }
       ]
    };
    let mut cursor = my_coll.find(query, None).await?;
=======
    let query = doc! { "qty": doc! { "$not": doc! { "$gt": 5 }}};
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = my_coll.find(query, find_options).await?;
>>>>>>> 5d33bbf (post review)
    while let Some(Document) = cursor.try_next().await? {
       println!("{}", Document);
    }
    //end-logical

    //begin-element
    let query = doc! { "description": doc! { "$exists": true } };
<<<<<<< HEAD
    let mut cursor = my_coll.find(query, None);
=======
    let find_options = FindOptions::builder().build();
    let mut cursor = my_coll.find(query, find_options);
>>>>>>> 5d33bbf (post review)
    while let Some(Document) = cursor.try_next().await? {
       println!("{}", Document);
    }
    //end-element

    //begin-evaluation
    // $mod means "modulo" and returns the remainder after division
    let query = doc! { "qty": doc! { "$mod": [ 3, 0 ] } };
<<<<<<< HEAD
    let mut cursor = my_coll.find(query, None);
=======
    let find_options = FindOptions::builder().build();
    let mut cursor = my_coll.find(query, find_options);
>>>>>>> 5d33bbf (post review)
    while let Some(Document) = cursor.try_next().await? {
       println!("{}", Document);
    }
    //end-evaluation
}