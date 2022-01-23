use hyper::{Body, Method, Request};

pub struct Client {
    common_headers: Vec<Header>,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Client {
    pub fn new(api_key: String) -> Client {
        Self {
            common_headers: vec![Header {
                name: String::from("Authorization"),
                value: api_key,
            }],
        }
    }

    #[tokio::main]
    pub async fn query_database(
        self: Client,
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

        let result = builder.body(Body::from(body))?;

        // let req = Request::builder()
        // .method(Method::POST)
        // .uri("http://httpbin.org/post")
        // .header("content-type", "application/json")
        // .body(Body::from(r#"{"library":"hyper"}"#))?;

        Ok(())
    }
}
