use super::entity::Ad;
use mongodb::{bson::doc, sync::Client};

const DB_NAME: &str = "scraper_leboncoin";
const COLLECTION_NAME: &str = "ads";

fn get_collection(client: &Client) -> mongodb::sync::Collection<Ad> {
    client.database(DB_NAME).collection(COLLECTION_NAME)
}

pub fn persist_ads(client: &Client, ads: &Vec<Ad>) {
    let collection = get_collection(&client);

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

    match collection.insert_many(unique_documents, None) {
        Ok(result) => println!("Persisted {} ads", result.inserted_ids.len()),
        Err(e) => println!("Failed to persist ads: {}", e),
    }
}

fn is_duplicated(client: &Client, ad: &Ad) -> bool {
    let collection = get_collection(&client);

    let date_iso_8601 = ad
        .publication_date
        .and_then(|date| Some(date.format("%Y-%m-%dT%H:%M:%S").to_string()));

    let filter = doc! {
        "title": &ad.title,
        "price": ad.price.and_then(|price| Some(price as u32)),
        "location":{
            "city_name": &ad.location.as_ref().and_then(|loc| loc.city_name.as_ref()),
            "postal_code": &ad.location.as_ref().and_then(|loc| loc.postal_code.as_ref()),
            },
        "price_per_square_meter": ad.price_per_square_meter,
        "surface": ad.surface.and_then(|surface| Some(surface as u32)),
        "seller_type": ad.seller_type.to_string(),
        "publication_date": date_iso_8601,
        "url": &ad.url,
    };

    collection.find_one(filter, None).unwrap().is_some()
}
