use hyper::{Body, Method, Request, Response};

pub trait Base {
    fn new(api_key: String) -> Self;
    fn get(&self, header: String, url: String, body: String) -> Response<()>;
    fn post(&self, header: String) -> Response<()>;
}
pub struct Default {
    api_key: String,
}

impl Base for Default {
    fn new(api_key: String) -> Self {
        Default { api_key }
    }

    fn get(&self, header: String, url: String, body: String) -> Response<()> {
        return Response::new(());
    }
    fn post(&self, header: String) -> Response<()> {
        return Response::new(());
    }
}
