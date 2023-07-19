use chrono::{ Utc };
use mongodb::{ Client, Collection };
use serde::{ Deserialize, Serialize };

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri: &str = "<connection string>";

    let client: Client = Client::with_uri_str(uri).await?;

    // begin-insert
    let my_coll: Collection<Document> = client.database("db").collection("site_users");

    let docs = vec![
        doc! { "name": "Sonya Mehta", "interests": vec!["fiction", "mystery", "memoir"], "last_active": Utc.with_ymd_and_hms(2019, 5, 13, 0, 0, 0).unwrap() },
        doc! { "name": "Selena Sun", "interests": vec!["fiction", "literary", "romance"], "last_active": Utc.with_ymd_and_hms(2019, 5, 25, 0, 0, 0).unwrap() },
        doc! { "name": "Carter Johnson", "interests": vec!["literary", "self help"], "last_active": Utc.with_ymd_and_hms(2019, 5, 31, 0, 0, 0).unwrap() },
        doc! { "name": "Rick Cortes", "interests": vec!["sci-fi", "fantasy", "memoir"], "last_active": Utc.with_ymd_and_hms(2019, 7, 1, 0, 0, 0).unwrap() },
        doc! { "name": "Belinda James", "interests": vec!["literary", "nonfiction"], "last_active": Utc.with_ymd_and_hms(2019, 6, 11, 0, 0, 0).unwrap() },
        doc! { "name": "Corey Saltz", "interests": vec!["fiction", "sports", "memoir"], "last_active": Utc.with_ymd_and_hms(2019, 1, 23, 0, 0, 0).unwrap() },
        doc! { "name": "Jan Soo", "interests": vec!["fiction", "sports"], "last_active": Utc.with_ymd_and_hms(2019, 1, 3, 0, 0, 0).unwrap() },
        doc! { "name": "Lisa Ray", "interests": vec!["poetry", "art", "memoir"], "last_active": Utc.with_ymd_and_hms(2019, 5, 30, 0, 0, 0).unwrap() },
        doc! { "name": "Kiran Murray", "interests": vec!["mystery", "fantasy", "memoir"], "last_active": Utc.with_ymd_and_hms(2019, 1, 30, 0, 0, 0).unwrap() },
        doc! { "name": "Beth Carson", "interests": vec!["horror", "mystery", "nonfiction"], "last_active": Utc.with_ymd_and_hms(2019, 8, 4, 0, 0, 0).unwrap() },
        doc! { "name": "Thalia Dorn", "interests": vec!["theory", "literary", "fiction"], "last_active": Utc.with_ymd_and_hms(2019, 8, 19, 0, 0, 0).unwrap() },
        doc! { "name": "Arthur Ray", "interests": vec!["graphic novel", "thriller", "fiction"], "last_active": Utc.with_ymd_and_hms(2019, 11, 27, 0, 0, 0).unwrap() }
    ];

    my_coll.insert_many(docs, None).await?;
    // end-insert

    Ok(())
}
