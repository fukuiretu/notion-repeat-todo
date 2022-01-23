use hyper::{Body, Client, Method, Request};

pub struct Header {
    pub name: String,
    pub value: String,
}

pub struct Database {
    common_headers: Vec<Header>,
}

impl Database {
    pub fn new(api_key: String) -> Database {
        Self {
            common_headers: vec![Header {
                name: String::from("Authorization"),
                value: api_key,
            }],
        }
    }

    #[tokio::main]
    pub async fn query(
        self: Database,
        headers: Vec<Header>,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("call query_catabase.");

        let mut builder = Request::builder()
            .method(Method::POST)
            .uri("http://httpbin.org/post");

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
