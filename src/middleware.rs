use log::info;
use rouille::{Request, Response};

const SERVER_NAME: &str = concat!("Lightweight Maven Repository v", env!("CARGO_PKG_VERSION"));

pub fn transform_response(response: Response) -> Response {
    response.with_unique_header("Server", SERVER_NAME)
}

fn print_request(request: &Request) {
    info!("{} {}", request.method(), request.url());
}

pub fn peek_request(request: &Request) {
    print_request(&request);
}
