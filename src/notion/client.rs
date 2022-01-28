use hyper::{Body, Client, Method, Request, Response};

trait Client {
    fn new(api_key: String) -> Self;

    fn get(&self, header: String) -> Response {
        return;
    }

    fn post(&self, header: String) -> Response {
        return;
    }
}

impl Default for Client {}
