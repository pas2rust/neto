use reqwest::Response;

use crate::components::data::{Body, Http, Query};

impl Http {
    pub async fn put<Path: Into<String>>(
        &self,
        path: Path,
        query: Query,
        body: Body,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let client = &self.client;
        let url = self.url(path);
        let req = client.put(url).query(&query);
        let req = self.body(req, body);

        let resp = req.send().await?;
        Ok(resp)
    }
}
