use crate::{ad::entity::Ads, database, services};

pub async fn app() {
    let client = database::service::init().await.unwrap();

    let ads: Ads = services::data_getter::get_data();

    let number_of_ads = ads.len();

    for ad in ads {
        println!("{}", ad);
    }

    println!("total ads: {}", number_of_ads);
}
