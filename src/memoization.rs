use std::fs;

use crate::html_query;

const FILE_PATH: &str = "saved_web_pages.txt";

pub async fn read_from_file_or_get_online() -> String {
    return match fs::read_to_string(&FILE_PATH) {
        Ok(content) => {
            println!("Using saved web page 📜");
            content
        }
        Err(err) => {
            println!("Erreur lors de la lecture du fichier : {}", err);
            html_query::get_html_online().await.unwrap()
        }
    };
}
