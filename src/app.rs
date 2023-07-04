use crate::{ad::Ad, services};

pub fn app() {
    let ads: Vec<Ad> = services::data_getter::get_data();

    let number_of_ads = ads.len();

    for ad in ads {
        println!("{}", ad);
    }

    println!("total ads: {}", number_of_ads);
}
