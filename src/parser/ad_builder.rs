use crate::ad::entity::Ad;

use super::parser;
use scraper::{element_ref::Text, ElementRef};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    iter::Enumerate,
};

fn compute_surface(price: u64, price_per_square_meter: u64) -> u64 {
    price / price_per_square_meter
}

fn get_hash(ad: &Ad) -> Option<String> {
    let mut hasher = DefaultHasher::new();
    ad.hash(&mut hasher);
    Some(hasher.finish().to_string())
}

pub fn get_ad_from_children_with_text(
    children_with_text: Enumerate<Text<'_>>,
    element: ElementRef<'_>,
) -> Ad {
    let mut ad = Ad::new_empty();

    ad.title = parser::try_get_title(element);
    ad.url = parser::try_get_ad_url(element).map(|url| url.to_string());

    for child_with_text in children_with_text {
        let childs_text = child_with_text.1;

        match parser::find_data_type(childs_text) {
            parser::DataType::Location(location) => {
                ad.location = Some(location);
            }
            parser::DataType::Price(price) => {
                ad.price = Some(price);
            }
            parser::DataType::PricePerSquare(square) => {
                ad.price_per_square_meter = Some(square);
            }
            parser::DataType::SellerType(seller_type) => {
                ad.seller_type = seller_type;
            }
            parser::DataType::PublicationDate(date) => {
                ad.publication_date = Some(date);
            }
            parser::DataType::None => {}
        }
    }
    if ad.price.is_some() && ad.price_per_square_meter.is_some() {
        ad.surface = Some(compute_surface(
            ad.price.unwrap(),
            ad.price_per_square_meter.unwrap() as u64,
        ))
    }
    ad.hash_value = get_hash(&ad);

    ad
}
