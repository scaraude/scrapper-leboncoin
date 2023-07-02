mod ad;
mod helper;
mod memoization;
mod pagined_website_explorator;
mod parser;
extern crate dotenv;
extern crate scraper;
use crate::pagined_website_explorator::PaginedWebsite;
use ad::Ad;
use dotenv::dotenv;
use scraper::{element_ref::Text, Html, Selector};
use std::{iter::Enumerate, vec};
use time::Instant;

const BASE_URL: &str = "https://www.leboncoin.fr/recherche";

fn get_ad_from_children_with_text(children_with_text: Enumerate<Text<'_>>) -> Ad {
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

fn main() {
    let start = Instant::now();

    dotenv().unwrap_or_else(|err| {
        panic!(
            "Erreur lors du chargement des variables d'environnement : {}",
            err
        );
    });

    let mut ads: Vec<Ad> = vec![];

    let selector = Selector::parse(r#"a[data-test-id="ad"]"#).unwrap();

    for webpage in PaginedWebsite::new(
        BASE_URL,
        helper::get_url_params_from_file(),
        helper::get_headers(),
    ) {
        let document = Html::parse_document(webpage.unwrap().as_str());

        for element in document.select(&selector) {
            let children_with_text = element.text().enumerate();
            ads.push(get_ad_from_children_with_text(children_with_text));
        }
    }

    let number_of_ads = ads.len();

    for ad in ads {
        println!("{}", ad);
    }

    println!("total ads: {}", number_of_ads);

    let elapsed = start.elapsed();
    println!("Temps écoulé: {:?}", elapsed);
}
