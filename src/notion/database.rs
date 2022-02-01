use super::client;

pub struct Header {
    pub name: String,
    pub value: String,
}

pub struct Database<T: client::Client> {
    client: T,
}

impl<T: client::Client> Database<T> {
    const QUERY_BASE_URL: &'static str = "https://api.notion.com/v1/databases/{database_id}/query";

    pub fn new(client: T) -> Database<T> {
        Self { client }
    }

    #[tokio::main]
    pub async fn query(
        self: Database<T>,
        database_id: String,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("call query_catabase.");

        self.client.get(
            String::from(""),
            Self::QUERY_BASE_URL.replace("{database_id}", database_id.as_str()),
            body,
        );

        // let mut builder = Request::builder()
        //     .method(Method::POST)
        //     .uri(Self::QUERY_BASE_URL.replace("{database_id}", database_id.as_str()));

        // for header in self.common_headers {
        //     builder = builder.header(&header.name, &header.value);
        //     println!("header.name:{}, header.value:{}", header.name, header.value);
        // }
        // for header in headers {
        //     builder = builder.header(&header.name, &header.value);
        //     println!("header.name:{}, header.value:{}", header.name, header.value);
        // }
        // let req = builder.body(Body::from(body))?;

        // let resp = Client::new().request(req).await?;
        // println!("Response: {}", resp.status());

        Ok(())
    }
}
