use std::fs;

use crate::html_query;

const FILE_PATH: &str = "saved_web_pages.txt";

pub async fn read_from_file_or_get_online() -> String {
    return match fs::read_to_string(&FILE_PATH) {
        Ok(content) => {
            println!("Using saved web page 📜");
            content
        }
        Err(_err) => {
            println!("No record found ❌");
            let web_page = html_query::get_html_online().await.unwrap();

            match save_web_page(web_page.clone()).await {
                Ok(_) => println!("web page saved 💾"),
                Err(_) => println!("Error while saving the page ❌"),
            }

            web_page
        }
    };
}

pub async fn save_web_page(web_page: String) -> std::io::Result<()> {
    fs::write(FILE_PATH, web_page)
}
