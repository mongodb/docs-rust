use mongodb::{ 
    bson::{ Document, doc }, 
    Client, 
    Collection, 
    options::{ FindOptions, FindOneOptions } 
};
use chrono::{ TimeZone, Utc };
use futures::TryStreamExt;
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Document> = client.database("sample_mflix").collection("theaters");

    // start-idx
    let index = IndexModel::builder()
        .keys(doc! { "location.geo": "2dsphere" })
        .build();

    let idx = my_coll.create_index(index, None).await?;
    println!("Created index:\n{}", idx.index_name);
    // end-idx

    // start-proximity
    let query = doc! {"location.geo": 
        doc! { "$near": 
            doc! {"$geometry": 
                doc! {"type": "Point"}, doc! { "coordinates": []float64{-93.24565, 44.85466}}
            },
            doc! {"$maxDistance": 1000},
        }
    }

    filter := bson.D{
        {"location.geo", bson.D{
            {"$near", bson.D{
                {"$geometry", query},
                {"$maxDistance", 1000},
            }},
        }},
    }
    var places []bson.D
    output, err := coll.Find(context.TODO(), filter)
    if err = output.All(context.TODO(), &places); err != nil {
        panic(err)
    }
  
    for _, place := range places {
        res, _ := bson.MarshalExtJSON(place, false, false)
        fmt.Println(string(res))
    }
    // end-proximity