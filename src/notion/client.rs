use reqwest;

pub trait Base {
    const NOTION_VERSION: &'static str = "2021-08-16";

    fn new(api_key: String) -> Self;
    fn get(&self, url: String, body: String) -> Result<(), reqwest::Error>;
    fn post(&self, url: String, data: String) -> Result<(), reqwest::Error>;

    fn client(&self) -> reqwest::Client {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Notion-Version",
            reqwest::header::HeaderValue::from_static(Self::NOTION_VERSION),
        );

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap()
    }
}
pub struct Default {
    api_key: String,
}

impl Base for Default {
    fn new(api_key: String) -> Self {
        Default { api_key }
    }

    fn get(&self, url: String, data: String) -> Result<(), reqwest::Error> {
        Ok(())
    }

    #[tokio::main]
    async fn post(&self, url: String, data: String) -> Result<(), reqwest::Error> {
        let res = self
            .client()
            .post(url)
            .header("Authorization", format!("Bearer '{}'", self.api_key))
            .body(data)
            .send()
            .await?;

        Ok(())
    }
}
