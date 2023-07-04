use std::{env, time::Instant};

use scraper::{Html, Selector};

use crate::{
    ad::entity::Ad, helper, pagined_website_explorator::PaginedWebsite, parser::ad_builder,
};

pub fn get_data() -> Vec<Ad> {
    let mut ads: Vec<Ad> = vec![];

    let selector = Selector::parse(r#"a[data-test-id="ad"]"#).unwrap();

    let pagined_website = PaginedWebsite::new(
        env::var("BASE_URL").unwrap().as_str(),
        helper::get_url_params_from_file(),
        helper::get_headers(),
    );

    for webpage in pagined_website {
        let wabpage_analyse_start = Instant::now();

        let document = Html::parse_document(webpage.unwrap().as_str());

        for element in document.select(&selector) {
            let children_with_text = element.text().enumerate();

            ads.push(ad_builder::get_ad_from_children_with_text(
                children_with_text,
                element,
            ));
        }
        println!(
            "wabpage analyse execution time: {:?}",
            wabpage_analyse_start.elapsed()
        );
    }
    ads
}
