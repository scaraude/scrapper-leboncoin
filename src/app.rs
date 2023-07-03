use crate::{ad::Ad, helper, pagined_website_explorator::PaginedWebsite, parser::ad_builder};
use scraper::{Html, Selector};

const BASE_URL: &str = "https://www.leboncoin.fr/recherche";

pub fn app() {
    let mut ads: Vec<Ad> = vec![];

    let selector = Selector::parse(r#"a[data-test-id="ad"]"#).unwrap();

    let pagined_website = PaginedWebsite::new(
        BASE_URL,
        helper::get_url_params_from_file(),
        helper::get_headers(),
    );

    for webpage in pagined_website {
        let document = Html::parse_document(webpage.unwrap().as_str());

        for element in document.select(&selector) {
            let children_with_text = element.text().enumerate();

            ads.push(ad_builder::get_ad_from_children_with_text(
                children_with_text,
                element,
            ));
        }
    }

    let number_of_ads = ads.len();

    for ad in ads {
        println!("{}", ad);
    }

    println!("total ads: {}", number_of_ads);
}
