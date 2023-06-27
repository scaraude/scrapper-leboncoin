mod ad;
mod html_query;
mod memoization;
mod parser;

extern crate dotenv;
extern crate scraper;

use ad::Ad;
use dotenv::dotenv;
use scraper::{Html, Selector};
use time::Instant;
use tokio::runtime::Runtime;

fn main() {
    let start = Instant::now();
    let rt = Runtime::new().unwrap();

    dotenv().unwrap_or_else(|err| {
        panic!(
            "Erreur lors du chargement des variables d'environnement : {}",
            err
        );
    });

    rt.block_on(async {
        let body = memoization::read_from_file_or_get_online().await;

        let document = Html::parse_document(body.as_str());

        let selector = Selector::parse(r#"a[data-test-id="ad"]"#).unwrap();

        for element in document.select(&selector) {
            let children_with_text = element.text().enumerate();

            let mut ad = Ad::new(None, None, None);

            for child_with_text in children_with_text {
                let childs_text = child_with_text.1;

                ad = parser::parse_data_from_ad(childs_text, ad);
            }

            println!("{}", ad)
        }
    });

    let elapsed = start.elapsed();
    println!("Temps écoulé: {:?}", elapsed);
}
