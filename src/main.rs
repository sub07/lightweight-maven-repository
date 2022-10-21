use crate::error::{Error, Result};
use crate::repo::RepoService;

mod error;
mod repo;
mod handler;
mod middleware;
mod model;
mod response_utils;

fn main() -> Result<()> {
    log4rs::init_file(
        std::env::var("log4rs_conf").ok().unwrap_or("log4rs.yaml".to_string()),
        Default::default(),
    ).unwrap();

    let repo_service = match RepoService::new() {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    rouille::start_server("127.0.0.1:3000", move |request| {
        middleware::peek_request(request);

        let repo_service = repo_service.clone();

        let response = match request.method() {
            "GET" => handler::handle_get(request, repo_service),
            "PUT" => handler::handle_put(request, repo_service),
            "HEAD" => handler::handle_head(request, repo_service),
            _ => response_utils::with_code(403),
        };

        middleware::transform_response(response)
    });
}
