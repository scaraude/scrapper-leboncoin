use reqwest::header::HeaderMap;
use serde_json::{Map, Value};
use url::Url;

pub struct WebExplorator {
    client: reqwest::blocking::Client,
    headers: HeaderMap,
    base_url: String,
    params: Map<String, Value>,
    page: u32,
    total_page: u32,
}

impl WebExplorator {
    pub fn new(base_url: &str, params: Map<String, Value>, headers: HeaderMap) -> WebExplorator {
        WebExplorator {
            params,
            headers,
            client: reqwest::blocking::Client::new(),
            base_url: base_url.to_owned(),
            page: 0,
            total_page: 0,
        }
    }

    fn format_url(&self) -> String {
        let mut url = Url::parse(&self.base_url).expect("Invalid base URL");

        for (key, value) in self.params.to_owned() {
            let value_string: String = match value {
                Value::String(s) => s,
                _ => panic!("Unexpected value type"),
            };

            url.query_pairs_mut()
                .append_pair(key.as_str(), value_string.as_str());
        }
        url.to_string()
    }

    fn get_one_page(&self) -> reqwest::Result<String> {
        self.client
            .get(self.format_url())
            .headers(self.headers.to_owned())
            .send()
            .unwrap()
            .text()
    }

    fn try_next(&mut self) -> reqwest::Result<Option<String>> {
        if self.page >= self.total_page {
            return Ok(None);
        }

        self.page += 1;

        Ok(Some(self.get_one_page().unwrap()))
    }
}

impl Iterator for WebExplorator {
    type Item = reqwest::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(html_text)) => Some(Ok(html_text)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
