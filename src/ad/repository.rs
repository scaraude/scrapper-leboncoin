// use super::entity::Ad;
// use mongodb::bson::{doc, Document};

// const DB_NAME: &str = "scraper_leboncoin";
// const COLLECTION_NAME: &str = "Ads";

// pub async fn persist_ads(client: &mongodb::Client, ads: Vec<Ad>) {
//     let db = client.database(DB_NAME);
//     let collection = db.collection::<Document>(COLLECTION_NAME);

//     let docs = vec![
//         doc! { "title": "1984", "author": "George Orwell" },
//         doc! { "title": "Animal Farm", "author": "George Orwell" },
//         doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//     ];

//     collection.insert_many(ads, None).await.unwrap();
// }
