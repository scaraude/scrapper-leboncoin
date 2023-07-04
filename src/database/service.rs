use mongodb::{bson::doc, options::ClientOptions, Client};

pub async fn init() -> mongodb::error::Result<mongodb::Client> {
    let database_url = format!(
        "mongodb+srv://{}:{}@cluster0.uanhfot.mongodb.net/?retryWrites=true&w=majority",
        std::env::var("DATABASE_USERNAME").unwrap(),
        std::env::var("DATABASE_PASSWORD").unwrap()
    );
    let client_options = ClientOptions::parse(database_url).await?;

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(client)
}
