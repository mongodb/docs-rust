use chrono::{ Utc, TimeZone };
use mongodb::{ Client, Collection };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
// start-struct
struct User {
    name: String,
    interests: Vec<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    last_active: chrono::DateTime<Utc>,
}
// end-struct

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri: &str = "<connection string>";

    let client: Client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<User> = client.database("test-db").collection("site-users");

    let user1 = User {
        name: "Sonya Mehta".to_string(),
        interests: vec!["fiction".to_string(), "mystery".to_string(), "memoir".to_string()],
        last_active: Utc.with_ymd_and_hms(2019, 5, 13, 8, 32, 12).unwrap(),
    };

    let result = my_coll.insert_one(user1, None).await?;
    let ins_id = result.inserted_id
        .as_object_id()
        .expect("Retrieved _id should have been of type ObjectId");
    println!("document ID: {:?}", ins_id);

    Ok(())
}
