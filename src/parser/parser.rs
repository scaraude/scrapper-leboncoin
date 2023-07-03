use crate::ad::{Location, SellerType};
use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError};

pub enum DataType {
    Price(u64),
    PricePerSquare(u32),
    Location(Location),
    SellerType(SellerType),
    PublicationDate(NaiveDateTime),
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

fn translate_month_to_number(french_month: &str) -> Option<usize> {
    let dict_month_french_english: HashMap<&str, usize> = [
        ("janvier", 1),
        ("février", 2),
        ("mars", 3),
        ("avril", 4),
        ("mai", 5),
        ("juin", 6),
        ("juillet", 7),
        ("août", 8),
        ("septembre", 9),
        ("octobre", 10),
        ("novembre", 11),
        ("décembre", 12),
    ]
    .iter()
    .cloned()
    .collect();

    dict_month_french_english.get(french_month).cloned()
}

fn check_is_date(input: &str) -> bool {
    let regex = Regex::new(r"(?i)^(Hier|Aujourd'hui|\d{1,2} \w+), (\d{2}:\d{2})$").unwrap();
    regex.is_match(input)
}

fn find_date_in_past(
    day: u32,
    month_in_number: u32,
    time: NaiveTime,
    now: chrono::NaiveDate,
) -> Result<NaiveDateTime, String> {
    let mut time_cursor = now
        .with_day(day)
        .expect("Failed te change day in date")
        .and_time(time);

    if now.month() == month_in_number {
        return Ok(time_cursor);
    }

    for _i in 0..13 {
        time_cursor -= chrono::Duration::days(28);

        if now.month() == month_in_number {
            return Ok(NaiveDateTime::new(
                now.with_day(day).expect("Failed te change day in date"),
                time,
            ));
        }
    }

    Err(String::from("failed to find a date"))
}

fn parse_relative_date(
    captures: regex::Captures<'_>,
    now: chrono::NaiveDate,
) -> Result<NaiveDateTime, String> {
    let relative_day = captures
        .name("relative_day")
        .expect("Failed to capture relative day")
        .as_str();
    let time_str = captures
        .name("time_str")
        .expect("Failed to capture time string")
        .as_str();

    let datetime_str = match relative_day {
        "Hier" => {
            let yesterday = now - Duration::days(1);
            format!("{} {}", yesterday, time_str)
        }
        "Aujourd'hui" => {
            format!("{} {}", now, time_str)
        }
        _ => return Err("Unexpected relative day".to_string()),
    };

    return match NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M") {
        Ok(datetime) => Ok(datetime),
        Err(_) => Err(String::from("Failed to parse date")),
    };
}

fn parse_french_date(
    captures: regex::Captures<'_>,
    now: chrono::NaiveDate,
) -> Result<NaiveDateTime, String> {
    let date = captures
        .name("date")
        .ok_or("Failed to capture date")?
        .as_str()
        .parse::<u32>()
        .expect("Failed to parse date in number");
    let month = captures
        .name("month")
        .ok_or("Failed to capture month")?
        .as_str();
    let time = captures
        .name("time")
        .ok_or("Failed to capture time")?
        .as_str();
    let time_as_naive_time =
        NaiveTime::parse_from_str(time, "%H:%M").expect("Failed to parse time");

    let month_in_number =
        TryInto::<u32>::try_into(translate_month_to_number(month).ok_or("Failed to parse month")?)
            .expect("Failed to parse month in number");

    return find_date_in_past(date, month_in_number, time_as_naive_time, now);
}

fn parse_date(date: &str) -> Result<NaiveDateTime, String> {
    let regex_relative_date =
        Regex::new(r"(?i)^(?P<relative_day>Hier|Aujourd'hui), (?P<time_str>\d{2}:\d{2})$").unwrap();

    let now: chrono::NaiveDate = chrono::Local::now().naive_local().date();

    if let Some(captures) = regex_relative_date.captures(date) {
        return parse_relative_date(captures, now);
    }

    let regex_date =
        Regex::new(r"(?i)^(?P<date>\d{1,2}) (?P<month>\w+), (?P<time>\d{2}:\d{2})$").unwrap();

    if let Some(captures) = regex_date.captures(date) {
        return parse_french_date(captures, now);
    }

    return Err("Failed to parse date".to_string());
}

pub fn find_data_type(text: &str) -> DataType {
    if check_is_price(text) {
        DataType::Price(convert_price_string_to_u64(text).expect("Failed to parse price"))
    } else if check_is_price_per_meter_square(text) {
        DataType::PricePerSquare(
            convert_price_per_meter_square_string_to_u32(text)
                .expect("Failed to parse price per meter square"),
        )
    } else if check_is_location(text) {
        DataType::Location(parse_city_and_postal_code(text).expect("Failed to parse location"))
    } else if check_is_date(text) {
        match parse_date(text) {
            Ok(parsed_date) => DataType::PublicationDate(parsed_date),
            Err(e) => {
                println!("Failed to parse date: {}", e);
                DataType::None
            }
        }
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
