use super::client;

pub struct Database<T: client::Base> {
    client: T,
}

impl<T: client::Base> Database<T> {
    const QUERY_BASE_URL: &'static str = "https://api.notion.com/v1/databases/{database_id}/query";

    pub fn new(client: T) -> Database<T> {
        Self { client }
    }

    pub fn query(
        self: Database<T>,
        database_id: String,
        data: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("call query_catabase.");

        self.client.post(
            vec![],
            Self::QUERY_BASE_URL.replace("{database_id}", database_id.as_str()),
            data,
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
