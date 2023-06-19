mod ad;
mod html_query;
mod memoization;
mod parser;

use ad::Ad;
use scraper::{Html, Selector};
extern crate scraper;
use time::Instant;
use tokio::runtime::Runtime;

fn main() {
    let start = Instant::now();
    let rt = Runtime::new().unwrap();

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
