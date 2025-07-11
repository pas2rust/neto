use reqwest::{Client, header::HeaderMap};

use crate::components::data::Http;

impl Http {
    pub fn config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        for (key, value) in &self.headers {
            let mut value = value.clone();
            value.set_sensitive(true);
            headers.insert(key, value);
        }
        let client = Client::builder().default_headers(headers).build()?;
        self.client = client;
        Ok(())
    }
}
