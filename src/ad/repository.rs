use std::time::Instant;

use super::entity::Ad;
use mongodb::{bson::doc, sync::Client};

const DB_NAME: &str = "scraper_leboncoin";
const COLLECTION_NAME: &str = "ads";

fn get_collection(client: &Client) -> mongodb::sync::Collection<Ad> {
    client.database(DB_NAME).collection(COLLECTION_NAME)
}
fn get_unique_documents<'a>(client: &'a Client, ads: &'a Vec<Ad>) -> Vec<&'a Ad> {
    let mut unique_documents: Vec<&Ad> = vec![];
    let mut duplicated_document_count = 0;
    for ad in ads {
        if !is_duplicated(client, ad) {
            unique_documents.push(ad)
        } else {
            duplicated_document_count += 1;
        }
    }
    println!("duplicated document count: {}", duplicated_document_count);

    unique_documents
}

pub fn persist_ads(client: &Client, ads: &Vec<Ad>) {
    let collection = get_collection(&client);

    let duplication_analyse_start = Instant::now();
    let unique_documents: Vec<&Ad> = get_unique_documents(client, ads);
    println!(
        "Duplication analyse execution time: {:?}",
        duplication_analyse_start.elapsed()
    );

    match collection.insert_many(unique_documents, None) {
        Ok(result) => println!("Persisted {} ads", result.inserted_ids.len()),
        Err(e) => println!("Failed to persist ads: {}", e),
    }
}

fn is_duplicated(client: &Client, ad: &Ad) -> bool {
    let collection = get_collection(&client);

    let duplicated_ad_suspected = collection
        .find(doc! { "hash_value": &ad.hash_value}, None)
        .unwrap();

    duplicated_ad_suspected
        .into_iter()
        .any(|suspected_ad_result| {
            if let Ok(suspected_ad) = suspected_ad_result {
                suspected_ad.url == ad.url && suspected_ad.title == ad.title
            } else {
                false
            }
        })
}

// #[cfg(test)]
// mod test {
//     use std::hash::{Hash, Hasher};
//     extern crate dotenv;

//     use dotenv::dotenv;

//     use mongodb::bson::doc;

//     use crate::database;

//     use super::get_collection;

//     fn get_hash(ad: &crate::ad::entity::Ad) -> Option<String> {
//         let mut hasher = std::collections::hash_map::DefaultHasher::new();
//         ad.hash(&mut hasher);
//         Some(hasher.finish().to_string())
//     }

//     #[test]
//     fn test() {
//         dotenv().unwrap_or_else(|err| {
//             panic!(
//                 "Erreur lors du chargement des variables d'environnement : {}",
//                 err
//             );
//         });

//         let client = database::service::init().unwrap();

//         let collection = get_collection(&client);

//         let all_documents_in_collection = collection.find(None, None).unwrap();

//         for document in all_documents_in_collection {
//             for document in all_documents_in_collection {
//                 if let Ok(mut ad) = document {
//                     if ad.hash_value.is_none() {
//                         ad.hash_value = get_hash(&ad);
//                         let update = doc! { "$set": { "hash_value": ad.hash_value }};
//                         collection.update_one(filter, update, None).unwrap();
//                     }
//                 }
//         }
//     }
// }
