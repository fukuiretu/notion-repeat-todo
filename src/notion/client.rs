use reqwest;

pub struct RequestHeader {
    pub name: String,
    pub value: String,
}

pub trait Base {
    fn new(api_key: String) -> Self;
    fn get(
        &self,
        headers: Vec<RequestHeader>,
        url: String,
        body: String,
    ) -> Result<(), reqwest::Error>;
    fn post(
        &self,
        headers: Vec<RequestHeader>,
        url: String,
        data: String,
    ) -> Result<(), Box<dyn std::error::Error>>;

    fn client(&self) -> reqwest::Client {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Notion-Version",
            reqwest::header::HeaderValue::from_static("2021-08-16"),
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

    fn get(
        &self,
        headers: Vec<RequestHeader>,
        url: String,
        body: String,
    ) -> Result<(), reqwest::Error> {
        Ok(())
    }

    #[tokio::main]
    async fn post(
        &self,
        headers: Vec<RequestHeader>,
        url: String,
        data: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client()
            .post(url)
            .header("Authorization", format!("Bearer '{}'", self.api_key))
            .body(data)
            .send()
            .await?;

        Ok(())
    }
}
