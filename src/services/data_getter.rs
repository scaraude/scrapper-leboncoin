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
        let webpage_analyse_start = Instant::now();

        if webpage.is_err() {
            continue;
        }

        let mut has_element = false;

        let document = Html::parse_document(webpage.unwrap().as_str());

        let selected_elements: scraper::html::Select<'_, '_> = document.select(&selector);

        for element in selected_elements {
            has_element = true;
            let children_with_text = element.text().enumerate();

            let ad = ad_builder::get_ad_from_children_with_text(children_with_text, element);
            ads.push(ad)
        }

        if !has_element {
            println!("No more annonce... 🙅‍♂️");
            break;
        }

        println!(
            "webpage analyse execution time: {:?}",
            webpage_analyse_start.elapsed()
        );
    }
    ads
}
