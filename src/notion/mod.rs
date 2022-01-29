use hyper::{Body, Client, Method, Request};
mod client;

pub struct Header {
    pub name: String,
    pub value: String,
}

pub struct Database {
    common_headers: Vec<Header>,
}

impl Database {
    const QUERY_BASE_URL: &'static str = "https://api.notion.com/v1/databases/{database_id}/query";

    pub fn new(api_key: String) -> Database {
        Self {
            common_headers: vec![
                Header {
                    name: String::from("Authorization"),
                    value: api_key,
                },
                Header {
                    name: String::from("Notion-Version"),
                    value: String::from("2021-08-16"),
                },
                Header {
                    name: String::from("Content-Type"),
                    value: String::from("application/json"),
                },
            ],
        }
    }

    #[tokio::main]
    pub async fn query(
        self: Database,
        database_id: String,
        headers: Vec<Header>,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("call query_catabase.");

        let mut builder = Request::builder()
            .method(Method::POST)
            .uri(Self::QUERY_BASE_URL.replace("{database_id}", database_id.as_str()));

        for header in self.common_headers {
            builder = builder.header(&header.name, &header.value);
            println!("header.name:{}, header.value:{}", header.name, header.value);
        }
        for header in headers {
            builder = builder.header(&header.name, &header.value);
            println!("header.name:{}, header.value:{}", header.name, header.value);
        }
        let req = builder.body(Body::from(body))?;

        let resp = Client::new().request(req).await?;
        println!("Response: {}", resp.status());

        Ok(())
    }
}
