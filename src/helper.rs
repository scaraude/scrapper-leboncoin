use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, COOKIE as HEADER_COOKIE, USER_AGENT};
use serde_json::{Map, Value};
use std::io::Read;
use std::{env, fs::File};

// TO USE:
// struct UrlParams {
//     category: String,
//     locations: String,
//     real_estate_type: String,
//     price: String,
//     square: String,
//     rooms: String,
//     page: String,
// }

pub fn get_url_params_from_file() -> Map<String, Value> {
    let mut file =
        File::open("url_params.json").expect("Impossible d'ouvrir le fichier de configuration");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Impossible de lire le fichier de configuration");

    serde_json::from_str(&contents)
        .expect("Erreur lors de la désérialisation du fichier de configuration")
}

pub fn get_headers() -> HeaderMap {
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
