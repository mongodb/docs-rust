use mongodb::{ bson::doc, options::{ ClientOptions, Credential, AuthMechanism }, Client };
use mongodb::options::oidc::{self, CallbackContext, IdpServerResponse};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let mut client_options = ClientOptions::parse(uri).await?;

    // start-ldap
    let plain_cred = Credential::builder()
        .username("<username>".to_string())
        .password("<password>".to_string())
        .mechanism(AuthMechanism::Plain)
        .source("$external".to_string())
        .build();

    client_options.credential = Some(plain_cred);
    let client = Client::with_options(client_options)?;
    // end-ldap

    // start-azure-imds
    client_options.credential = Credential::builder()
        .username("<username>".to_owned())
        .mechanism(AuthMechanism::MongoDbOidc)
        .mechanism_properties(
            doc! {"ENVIRONMENT": "azure", "TOKEN_RESOURCE": "<audience>"}
        )
        .build()
        .into(); // Convert the builder into a Credential object
    
    let client = Client::with_options(client_options)?;
    let res = client
        .database("test")
        .collection::<Document>("test")
        .find_one(doc! {})
        .await?;
    // end-azure-imds

    // start-gcp-imds
    opts.credential = Credential::builder()
        .mechanism(AuthMechanism::MongoDbOidc)
        .mechanism_properties(
            doc! {"ENVIRONMENT": "gcp", "TOKEN_RESOURCE": "<audience>"}
        )
        .build()
        .into();
    let client = Client::with_options(opts)?;
    let res = client
        .database("test")
        .collection::<Document>("test")
        .find_one(doc! {})
        .await?;
    // end-gcp-imds

    // start-custom-callback-machine
    opts.credential = Credential::builder()
    .mechanism(AuthMechanism::MongoDbOidc)
    .oidc_callback(oidc::Callback::machine(move |_| {
        async move {
            let token_file_path = std::env::var("AWS_WEB_IDENTITY_TOKEN_FILE")?;
            let access_token = tokio::fs::read_to_string(token_file_path).await?;
            Ok(IdpServerResponse {
                access_token,
                expires: None,
                refresh_token: None,
            })
        }
        .boxed()
    }))
    .build()
    .into();

    let client = Client::with_options(opts)?;

    let res = client
    .database("test")
    .collection::<bson::Document>("test")
    .find_one(doc! {}, None)
    .await?;
    // end-custom-callback-machine

    // start-custom-callback-user
    async fn cb(params: CallbackContext) -> mongodb::error::Result<IdpServerResponse> {
	    idp_info := params.idp_info.ok_or(Error::NoIDPInfo)?;
        let (access_token, expires, refresh_token) = negotiate_with_idp(ctx, idpInfo.Issuer).await?;
	        Ok(oidc::IdpServerResponse {
            access_token,
            expires: Some(expires),
            refresh_token: Some(refresh_token),
         })
    }
    opts.credential = Credential::builder()
            .mechanism(AuthMechanism::MongoDbOidc)
            .oidc_callback(oidc::Callback::human(move|c| {
                 async move { cb(c).await }.boxed()
            }))
            .build()
            .into();
    let res = client
            .database("test")
            .collection::<Document>("test")
            .find_one(doc! {})
            .await;
    // end-custom-callback-user

    Ok(())
}
