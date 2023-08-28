use mongodb::{ bson::doc, options::{ ClientOptions, Credential, AuthMechanism }, Client };

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let mut client_options = ClientOptions::parse(uri).await?;

    // start-default
    let default_cred = Credential::builder()
        .username("<username>".to_string())
        .password("<password>".to_string())
        .source("<auth_db>".to_string())
        .build();

    client_options.credential = Some(default_cred);
    let client = Client::with_options(client_options)?;
    // end-default

    // start-scramsha256
    let scram_sha_256_cred = Credential::builder()
        .username("<username>".to_string())
        .password("<password>".to_string())
        .mechanism(AuthMechanism::ScramSha256)
        .source("<auth_db>".to_string())
        .build();

    client_options.credential = Some(scram_sha_256_cred);
    let client = Client::with_options(client_options)?;
    // end-scramsha256

    // start-scramsha1
    let scram_sha_1_cred = Credential::builder()
        .username("<username>".to_string())
        .password("<password>".to_string())
        .mechanism(AuthMechanism::ScramSha1)
        .source("<auth_db>".to_string())
        .build();

    client_options.credential = Some(scram_sha_1_cred);
    let client = Client::with_options(client_options)?;
    // end-scramsha1

    Ok(())
}
