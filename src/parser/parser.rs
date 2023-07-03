use regex::Regex;
use std::num::ParseIntError;

use crate::ad::{Location, SellerType};

pub enum DataType {
    Price(u64),
    PricePerSquare(u32),
    Location(Location),
    SellerType(SellerType),
    None,
}

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

fn check_is_date(input: &str) -> bool {
    let regex = Regex::new(r"(?i)^(Hier|Aujourd'hui|\d{1,2} \w+), (\d{2}:\d{2})$").unwrap();
    regex.is_match(input)
}

fn convert_price_string_to_u64(price_as_string: &str) -> Result<u64, ParseIntError> {
    let cleaned_input = price_as_string.replace(['\u{a0}', '€'], "");
    cleaned_input.parse::<u64>()
}

fn convert_price_per_meter_square_string_to_u32(
    price_per_meter_square_as_string: &str,
) -> Result<u32, ParseIntError> {
    let cleaned_input: String = price_per_meter_square_as_string
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();

    cleaned_input.parse::<u32>()
}

fn parse_city_and_postal_code(location: &str) -> Option<Location> {
    let re = Regex::new(r"(?i)(?P<city_name>\b[a-zÀ-ÿ' -]+\b\s)+(?P<postal_code>\d{2}[ ]?\d{3}\b)")
        .unwrap();

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

fn check_is_professional(input: &str) -> bool {
    input.eq("Pro")
}

pub fn find_data_type(text: &str) -> DataType {
    if check_is_price(text) {
        DataType::Price(convert_price_string_to_u64(text).unwrap())
    } else if check_is_price_per_meter_square(text) {
        DataType::PricePerSquare(convert_price_per_meter_square_string_to_u32(text).unwrap())
    } else if check_is_location(text) {
        DataType::Location(parse_city_and_postal_code(text).unwrap())
    // } else if check_is_date(text) {
    // parse_date(text)
    } else if check_is_professional(text) {
        DataType::SellerType(SellerType::Professional)
    } else {
        DataType::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_is_pro_with_pro() {
        assert_eq!(check_is_professional("Pro"), true);
    }

    #[test]
    fn test_check_is_pro_with_non_pro() {
        assert_eq!(check_is_professional("NonPro"), false);
        assert_eq!(check_is_professional(" Pro"), false);
        assert_eq!(check_is_professional("Pro "), false);
        assert_eq!(check_is_professional("something with Pro in it"), false);
        assert_eq!(check_is_professional("somethingwithProinit"), false);
    }

    #[test]
    fn test_check_is_date() {
        assert_eq!(check_is_date("Hier, 12:34"), true);
        assert_eq!(check_is_date("Aujourd'hui, 23:59"), true);
        assert_eq!(check_is_date("31 Dec, 00:00"), true);
        assert_eq!(check_is_date("31 Mai, 03:00"), true);
        assert_eq!(check_is_date("Invalid date"), false);
        assert_eq!(check_is_date("Invalid date with 11:23"), false);
    }
}
