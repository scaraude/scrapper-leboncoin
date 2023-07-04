use crate::{ad::entity::Ads, database, services};

pub fn app() {
    match database::service::init() {
        Ok(_) => println!("Pinged your deployment. You successfully connected to MongoDB! ✅"),
        Err(_) => println!("Ping failed, you're not connected to DB... ❌"),
    }

    let ads: Ads = services::data_getter::get_data();

    let number_of_ads = ads.len();

    for ad in ads {
        println!("{}", ad);
    }

    println!("total ads: {}", number_of_ads);
}
