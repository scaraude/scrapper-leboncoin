mod ad;
mod helper;
mod memoization;
mod parser;
mod web_explorator;

extern crate dotenv;
extern crate scraper;

use std::vec;

use ad::Ad;
use dotenv::dotenv;
use scraper::{Html, Selector};
use time::Instant;

use crate::web_explorator::WebExplorator;
const BASE_URL: &str = "https://www.leboncoin.fr/recherche";

fn main() {
    let start = Instant::now();

    dotenv().unwrap_or_else(|err| {
        panic!(
            "Erreur lors du chargement des variables d'environnement : {}",
            err
        );
    });

    let mut ads: Vec<Ad> = vec![];

    for webpage in WebExplorator::new(
        BASE_URL,
        helper::get_url_params_from_file(),
        helper::get_headers(),
    ) {
        let document = Html::parse_document(webpage.unwrap().as_str());

        let selector = Selector::parse(r#"a[data-test-id="ad"]"#).unwrap();

        for element in document.select(&selector) {
            let children_with_text = element.text().enumerate();

            let mut ad = Ad::new(None, None, None);

            for child_with_text in children_with_text {
                let childs_text = child_with_text.1;

                ad = parser::append_parsed_data(childs_text, ad);
            }

            ads.push(ad);
        }
    }

    println!(
        "-------------------------------- ADS
        {:?}`",
        ads
    );

    println!("total ads: {}", ads.len());

    let elapsed = start.elapsed();
    println!("Temps écoulé: {:?}", elapsed);
}
