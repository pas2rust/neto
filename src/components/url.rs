use crate::components::data::Http;

impl Http {
    pub fn url<Path: Into<String>>(&self, path: Path) -> String {
        let path = path.into();
        if self.base_url.ends_with('/') || path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        }
    }
}
