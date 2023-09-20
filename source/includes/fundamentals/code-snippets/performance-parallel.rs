let client = Client::with_uri_str("mongodb://example.com").await?;
let some_data = doc! { "title": "1984", "author": "George Orwell" };

for i in 0..5 {
    let client_ref = client.clone();

    task::spawn(async move {
        let collection = client_ref
            .database("items")
            .collection::<Document>(&format!("coll{}", i));

        collection.insert_one(some_data)
    });
}
