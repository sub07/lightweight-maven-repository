use rouille::{Request, Response};

use crate::response_utils;
use crate::handler::handle_artifact_error;
use crate::service::repo::RepoService;


pub fn handle(request: &Request, repo: RepoService) -> Response {
    match repo.get_artifact_size(request.url()) {
        Ok(size) => response_utils::with_content_size(size),
        Err(err) => handle_artifact_error(err),
    }
}
