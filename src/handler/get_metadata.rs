use rouille::{Request, Response};

use crate::response_utils;
use crate::handler::handle_error;
use crate::repo::RepoService;

pub fn handle(request: &Request, repo: RepoService) -> Response {
    match repo.generate_maven_metadata(request.url()) {
        Ok(xml) => response_utils::text(xml),
        Err(err) => handle_error(err),
    }
}
