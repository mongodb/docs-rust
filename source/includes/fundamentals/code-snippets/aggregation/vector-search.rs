#![recursion_limit = "2560"]
use bson::binary::Vector;
use mongodb::Collection;
use mongodb::{bson::doc, Client};

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Movie {
    title: String,
    plot: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<Movie> = client
        .database("sample_mflix")
        .collection("embedded_movies");

    // start-basic-query
    let query_vector = Vector::Float32(vec![-0.0016261312, -0.028070757, -0.011342932]);
    let pipeline = vec![
        doc! {
            "$vectorSearch": doc! {
            "queryVector": &query_vector,
            "path": "plot_embedding",
            "numCandidates": 150,
            "index": "vector_index",
            "limit": 5
        }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "title": 1,
                "plot": 1,
            }
        },
    ];

    let mut results = my_coll.aggregate(pipeline).await?;
    while let Some(result) = results.try_next().await? {
        println!("{}", result);
    }
    // end-basic-query

    // start-score-query
    let pipeline = vec![
        doc! {
            "$vectorSearch": doc! {
            "queryVector": &query_vector,
            "path": "plot_embedding",
            "numCandidates": 150,
            "index": "vector_index",
            "limit": 5
        }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "title": 1,
                "score": doc! { "$meta": "vectorSearchScore" },
            }
        },
    ];

    let mut results = my_coll.aggregate(pipeline).await?;
    while let Some(result) = results.try_next().await? {
        println!("{}", result);
    }
    // end-score-query

    Ok(())
}
