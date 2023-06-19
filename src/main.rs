mod ad;
mod html_query;

use ad::{Ad, Location};
use scraper::{Html, Selector};
extern crate scraper;
use regex::Regex;
use std::fs;
use std::num::ParseIntError;
use time::Instant;
use tokio::runtime::Runtime;

const FILE_PATH: &str = "saved_web_pages.txt";

async fn read_from_file_or_get_online() -> String {
    return match fs::read_to_string(&FILE_PATH) {
        Ok(content) => {
            println!("Using saved web page 📜");
            content
        }
        Err(err) => {
            println!("Erreur lors de la lecture du fichier : {}", err);
            html_query::get_html_online().await.unwrap()
        }
    };
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

fn format_data_from_ad(childs_text: &str, mut ad: Ad) -> Ad {
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

fn main() {
    let start = Instant::now();
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let body = read_from_file_or_get_online().await;

        let document = Html::parse_document(body.as_str());

        let selector = Selector::parse(r#"a[data-test-id="ad"]"#).unwrap();

        for element in document.select(&selector) {
            let children_with_text = element.text().enumerate();

            let mut ad = Ad::new(None, None, None);

            for child_with_text in children_with_text {
                let childs_text = child_with_text.1;

                ad = format_data_from_ad(childs_text, ad);
            }

            println!("{}", ad)
        }
    });

    let elapsed = start.elapsed();
    println!("Temps écoulé: {:?}", elapsed);
}
