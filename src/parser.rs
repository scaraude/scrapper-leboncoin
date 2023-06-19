use regex::Regex;
use std::num::ParseIntError;

use crate::ad::{Ad, Location};

fn check_is_price(input: &str) -> bool {
    let regex = Regex::new(r"\b(?:[1-9]\d{0,2}(?:[ \xa0]\d{3})*|0)(?:\.\d{1,2})?[ \xa0]*€(?:\s|$)")
        .unwrap();
    regex.is_match(input)
}

fn check_is_price_per_meter_square(input: &str) -> bool {
    input.ends_with("€/m²")
}

fn check_is_location(input: &str) -> bool {
    let regex = Regex::new(r"(?i)\b[a-zÀ-ÿ' -]+\b\s+\d{2}[ ]?\d{3}\b").unwrap();
    regex.is_match(input)
}

fn convert_price_string_to_u64(price_as_string: &str) -> Result<u64, ParseIntError> {
    let cleaned_input = price_as_string.replace(['\u{a0}', '€'], "");
    cleaned_input.parse::<u64>()
}

fn convert_price_per_meter_square_string_to_u64(
    price_per_meter_square_as_string: &str,
) -> Result<u32, ParseIntError> {
    let cleaned_input: String = price_per_meter_square_as_string
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();

    cleaned_input.parse::<u32>()
}

fn parse_city_and_postal_code(location: &str) -> Option<Location> {
    let re = Regex::new(r"^(?P<city_name>[^\d]+)\s+(?P<postal_code>\d{5})$").unwrap();

    if let Some(captures) = re.captures(location) {
        let city_name = captures
            .name("city_name")
            .unwrap()
            .as_str()
            .trim()
            .to_string();
        let postal_code = captures.name("postal_code").unwrap().as_str().to_string();
        Some(Location {
            city_name: Some(city_name),
            postal_code: Some(postal_code),
        })
    } else {
        None
    }
}

pub fn parse_data_from_ad(childs_text: &str, mut ad: Ad) -> Ad {
    match childs_text {
        childs_text if check_is_price(childs_text) => {
            (ad).price = convert_price_string_to_u64(childs_text).ok();
        }
        childs_text if check_is_price_per_meter_square(childs_text) => {
            ad.price_per_square_meter =
                convert_price_per_meter_square_string_to_u64(childs_text).ok();
        }
        childs_text if check_is_location(childs_text) => {
            ad.location = parse_city_and_postal_code(childs_text);
        }
        _ => {
            println!(
                "La chaîne '{}' ne correspond à aucune des datas collectées",
                childs_text
            );
        }
    }
    ad
}
