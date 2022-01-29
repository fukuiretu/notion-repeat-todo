use hyper::{Body, Method, Request, Response};

trait Client {
    fn get(&self, header: String) -> Response<()>;
    fn post(&self, header: String) -> Response<()>;
}

impl Default for dyn Client {
    fn get(&self, header: String) -> Response<()> {
        return Response::new(());
    }
    fn post(&self, header: String) -> Response<()> {
        return Response::new(());
    }
}
