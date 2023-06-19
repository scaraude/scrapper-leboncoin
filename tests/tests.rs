use std::collections::HashMap;
use url::Url;

#[test]
fn test_generate_url_with_params() {
    let base_url = "https://www.leboncoin.fr/recherche";
    let mut params = HashMap::new();
    params.insert("category", "9");
    params.insert("locations", "d_89,d_21");
    params.insert("real_estate_type", "1");
    params.insert("price", "200000-350000");
    params.insert("square", "180-max");
    params.insert("rooms", "8-max");
    params.insert("page", "19");

    let generated_url = generate_url_with_params(base_url, &params);

    let expected_url = "https://www.leboncoin.fr/recherche?category=9&locations=d_89%2Cd_21&real_estate_type=1&price=200000-350000&square=180-max&rooms=8-max&page=19";

    assert_eq!(generated_url, expected_url);
}

fn generate_url_with_params(base_url: &str, params: &HashMap<&str, &str>) -> String {
    let mut url = Url::parse(base_url).expect("Invalid base URL");

    for (key, value) in params {
        url.query_pairs_mut().append_pair(key, value);
    }

    url.to_string()
}
