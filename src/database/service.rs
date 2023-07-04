use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
};

pub fn init() -> mongodb::error::Result<()> {
    let database_uri = format!(
        "mongodb+srv://{}:{}@cluster0.uanhfot.mongodb.net/?retryWrites=true&w=majority",
        std::env::var("DATABASE_USERNAME").unwrap(),
        std::env::var("DATABASE_PASSWORD").unwrap()
    );
    let mut client_options = ClientOptions::parse(database_uri)?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)?;
    Ok(())
}
