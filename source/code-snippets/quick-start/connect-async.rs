use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client };

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
   // Replace the placeholder with your Atlas connection string
   let uri = "<connection string>";
   let mut client_options = ClientOptions::parse(uri).await?;

   // Set the server_api field of the client_options object to Stable API version 1
   let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
   client_options.server_api = Some(server_api);
   
   // Create a new client and connect to the server
   let client = Client::with_options(client_options)?;

   // Get a handle on the "movies" collection in the "sample_mflix" database
   let coll = client.database("sample_mflix").collection::<mongodb::options::UpdateModifications>("movies");

   //Find the movie "The Perils of Pauline" in the "movies" collection
   let my_movie = coll.find_one(doc! {"title" : "The Perils of Pauline" }, None).await?;

   //print the document that contains the movie found
   println!("{:?}", my_movie);
   Ok(())
}