use std::iter::Enumerate;

use scraper::element_ref::Text;

use crate::ad::Ad;

use super::parser;

pub fn get_ad_from_children_with_text(children_with_text: Enumerate<Text<'_>>) -> Ad {
    let mut ad = Ad::new(None, None, None);

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
            parser::DataType::None => {
                println!(
                    "La chaîne '{}' ne correspond à aucune des datas collectées",
                    childs_text
                );
            }
        }
    }
    ad
}
