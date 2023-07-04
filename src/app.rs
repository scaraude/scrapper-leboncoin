use crate::{ad::entity::Ads, database, services};

pub fn app() {
    let client = database::service::init().expect("Failed to init DB...");

    let ads: Ads = services::data_getter::get_data();

    crate::ad::repository::persist_ads(&client, &ads);

    println!("total ads: {}", ads.len());
}
