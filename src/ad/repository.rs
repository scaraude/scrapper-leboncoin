use super::entity::Ad;
use mongodb::sync::Client;

const DB_NAME: &str = "scraper_leboncoin";
const COLLECTION_NAME: &str = "ads";

pub fn persist_ads(client: &Client, ads: &Vec<Ad>) {
    let database = client.database(DB_NAME);
    let collection = database.collection::<Ad>(COLLECTION_NAME);

    let result = collection
        .insert_many(ads, None)
        .expect("Failed to persist ads");

    println!("Persisted {} ads", result.inserted_ids.len());
}
