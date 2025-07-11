use reqwest::Response;

use crate::components::data::{Http, Query};

impl Http {
    pub async fn get<Path: Into<String>>(
        &self,
        path: Path,
        query: Query,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let client = &self.client;
        let url = self.url(path);
        let resp = client.get(url).query(&query).send().await?;
        Ok(resp)
    }
}
