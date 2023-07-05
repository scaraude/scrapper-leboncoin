use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime};
use regex::Regex;

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

    for _i in 0..14 {
        time_cursor -= chrono::Duration::days(28);

        if time_cursor.month() == month_in_number {
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

pub fn parse_date(date: &str) -> Result<NaiveDateTime, String> {
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

pub fn check_is_date(input: &str) -> bool {
    let regex = Regex::new(r"(?i)^(Hier|Aujourd'hui|\d{1,2} \w+), (\d{2}:\d{2})$").unwrap();
    regex.is_match(input)
}
