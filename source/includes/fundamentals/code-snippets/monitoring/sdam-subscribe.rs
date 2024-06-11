use std::{ env, sync::Arc };

use bson::Document;
use mongodb::{
    Client,
    Collection,
    event::sdam::{ SdamEventHandler, ServerOpeningEvent },
    options::ClientOptions,
};

fn main() -> mongodb::error::Result<()> {
    // begin-sdam
    let mut client_options = ClientOptions::parse("<connection string>").await?;
    client_options.sdam_event_handler = Some(EventHandler::callback(|ev| println!("{:?}", ev)));
    
    let client = Client::with_options(client_options)?;

    // ... perform actions with the client to generate events

    // end-sdam
    Ok(())
}
