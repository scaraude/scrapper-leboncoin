use reqwest::{
    header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, COOKIE as HEADER_COOKIE, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
struct UrlParams {
    category: String,
    locations: String,
    real_estate_type: String,
    price: String,
    square: String,
    rooms: String,
    page: String,
}

const BASE_URL: &str = "https://www.leboncoin.fr/recherche";

fn get_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let user_agent = env::var("USER_AGENT").unwrap().parse().unwrap();
    let cookie = env::var("COOKIE").unwrap().parse().unwrap();

    headers.insert(USER_AGENT, user_agent);
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert(
        ACCEPT_LANGUAGE,
        "fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap(),
    );
    headers.insert(HEADER_COOKIE, cookie);

    return headers;
}

pub async fn get_html_online() -> Result<String, String> {
    let client = Client::new();

    let headers: HeaderMap = get_header();

    let url = generate_url_with_params();
    let response = client.get(url).headers(headers).send().await.unwrap();

    if response.status().is_success() {
        return Ok(response.text().await.unwrap());
    } else {
        let error_message = format!("La requête a échoué avec le code : {}", response.status());
        println!("{}", error_message);
        Err(error_message)
    }
}

fn get_url_params_from_file() -> Map<String, Value> {
    let mut file =
        File::open("url_params.json").expect("Impossible d'ouvrir le fichier de configuration");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Impossible de lire le fichier de configuration");

    serde_json::from_str(&contents)
        .expect("Erreur lors de la désérialisation du fichier de configuration")
}

fn generate_url_with_params() -> String {
    let mut url = Url::parse(BASE_URL).expect("Invalid base URL");
    let url_params: Map<String, Value> = get_url_params_from_file();

    for (key, value) in url_params {
        let value_string: String = match value {
            Value::String(s) => s,
            _ => panic!("Unexpected value type"),
        };

        url.query_pairs_mut()
            .append_pair(key.as_str(), value_string.as_str());
    }

    url.to_string()
}
