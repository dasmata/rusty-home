use reqwest::{Client, IntoUrl, Response};

#[derive(Default)]
pub struct HClient {
    transport: Client,
    key: String,
    path: String
}

impl HClient {
    pub fn new(path: &str, key: &str) -> Self {
        Self {
            transport: reqwest::Client::new(),
            path: String::from(path),
            key: String::from(key)
        }
    }

    pub async fn get<U: IntoUrl>(&self, uri: U) -> Result<Response, Box<dyn std::error::Error>> {
        let data = self.transport.get(format!("{}{}?access_token={}", self.path, uri.as_str(), self.key)).send().await?;

        Ok(data)
    }
}
