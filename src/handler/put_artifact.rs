use rouille::{Request, Response};

use crate::handler::{extract_data_from_request, handle_error};
use crate::repo::RepoService;
use crate::response_utils;

pub fn handle(request: &Request, repo: RepoService) -> Response {
    let data = match extract_data_from_request(request) {
        Ok(data) => data,
        Err(_) => return response_utils::internal_error(),
    };

    match repo.write_artifact(request.url(), data) {
        Ok(()) => response_utils::ok(),
        Err(err) => handle_error(err),
    }
}
