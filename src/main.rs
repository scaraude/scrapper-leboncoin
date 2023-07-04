mod ad;
mod app;
mod helper;
mod memoization;
mod pagined_website_explorator;
mod parser;
mod services;

extern crate dotenv;
extern crate scraper;
use dotenv::dotenv;
use time::Instant;

fn main() {
    dotenv().unwrap_or_else(|err| {
        panic!(
            "Erreur lors du chargement des variables d'environnement : {}",
            err
        );
    });

    let time_app_start = Instant::now();

    app::app();

    println!("App execution time: {:?}", time_app_start.elapsed());
}
