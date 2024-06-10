use std::{ env, sync::Arc };

use bson::Document;
use mongodb::{
    Client,
    Collection,
    event::sdam::{ SdamEventHandler, ServerOpeningEvent },
    options::ClientOptions,
};

fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";

    let mut client_options = ClientOptions::parse_async(uri).await?;

    // begin-sdam
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.sdam_event_handler = Some(EventHandler::callback(|ev| match ev {
        SdamEvent::ServerOpening(_) => {
            println!("{:?}", ev)
        }
        _ => (),
    }));
    
    let client = Client::with_options(client_options)?;

    // ... perform actions with the client to generate events

    // end-sdam
    Ok(())
}
