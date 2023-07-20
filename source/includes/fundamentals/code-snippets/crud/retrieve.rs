use bson::Document;
use chrono::{ TimeZone, Utc };
use futures::TryStreamExt;
use mongodb::{ bson::doc, Client, Collection, options::{ FindOptions, FindOneOptions } };
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    // begin-insert
    let my_coll: Collection<Document> = client.database("db").collection("inventory");

    let docs = vec![
        doc! { "item": "candle", "quantity": 101, "last_ordered": Utc.with_ymd_and_hms(2018, 1, 15, 0, 0, 0).unwrap() },
        doc! { "item": "basket", "quantity": 56, "last_ordered": Utc.with_ymd_and_hms(2017, 3, 15, 0, 0, 0).unwrap() },
        doc! { "item": "placemat", "quantity": 80, "last_ordered": Utc.with_ymd_and_hms(2018, 7, 15, 0, 0, 0).unwrap() },
        doc! { "item": "watering can", "quantity": 19, "last_ordered": Utc.with_ymd_and_hms(2017, 11, 1, 0, 0, 0).unwrap() },
        doc! { "item": "hanger", "quantity": 214, "last_ordered": Utc.with_ymd_and_hms(2016, 8, 1, 0, 0, 0).unwrap() }
    ];

    my_coll.insert_many(docs, None).await?;
    // end-insert

    // begin-find-many
    let opts: FindOptions = FindOptions::builder()
        .sort(doc! { "quantity": -1 })
        .build();

    let mut results = my_coll.find(
        doc! { "$and": vec!
            [
                doc! { "last_ordered": 
                    doc! { "$lt": Utc.with_ymd_and_hms(2018, 1, 1, 0, 0, 0).unwrap() }
                },
                doc! { "quantity": doc! { "$lt": 100 } }
            ] },
        opts
    ).await?;

    while let Some(result) = results.try_next().await? {
        let doc: Document = bson::from_document(result)?;
        println!("* {}", doc);
    }
    // end-find-many
    print!("\n");

    // begin-find-one
    let opts: FindOneOptions = FindOneOptions::builder().skip(2).build();
    let result = my_coll.find_one(doc! { "quantity": doc! { "$gte": 20 } }, opts).await?;

    println!("* {}", result.unwrap());
    // end-find-one
    print!("\n");

    // begin-agg
    let pipeline = vec![
        doc! { "$project": { "year_last_ordered" : doc! { "$year" : "$last_ordered" }, "quantity": 1 } },
        doc! { "$group": doc! { "_id" : doc! {"year_last_ordered": "$year_last_ordered"} ,
                                "remaining_qty" : doc! { "$sum" : "$quantity" } } },
        doc! { "$sort": { "_id.year" : 1 } }
    ];

    let mut results = my_coll.aggregate(pipeline, None).await?;
    while let Some(result) = results.try_next().await? {
        let doc: Document = bson::from_document(result)?;
        println!("* {}", doc);
    }
    // end-agg

    Ok(())
}
