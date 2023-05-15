use reqwest::header::{HeaderMap, ACCEPT_LANGUAGE, USER_AGENT};
use reqwest::Client;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();

        // Crée un en-tête personnalisé avec un agent utilisateur spécifique
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT_LANGUAGE,
            "fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap(),
        );
        headers.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36".parse().unwrap());

        // Effectue la requête GET avec les en-têtes personnalisés
        let response = client
            .get("https://www.leboncoin.fr")
            .headers(headers)
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let body = response.text().await.unwrap();
            println!("{}", body);
        } else {
            println!("La requête a échoué avec le code : {}", response.status());
        }
    });
}
