use rouille::{Request, Response};

use crate::handler::handle_artifact_error;
use crate::service::repo::RepoService;

use crate::response_utils;

pub fn handle(request: &Request, repo: RepoService) -> Response {
    match repo.read(request.url()) {
        Ok(bytes) => response_utils::bytes(bytes),
        Err(err) => handle_artifact_error(err),
    }
}
