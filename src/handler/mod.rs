use std::io::Read;

use rouille::{Request, Response};

use crate::{Error, response_utils, Result};
use crate::service::authentication::extract_authentication_info;
use crate::service::repo::RepoService;

mod get_artifact;
mod get_metadata;
mod head_artifact;
mod put_artifact;
mod put_metadata;

pub fn handle_put(request: &Request, repo: RepoService) -> Response {
    match extract_authentication_info(request) {
        Some(user) => {
            if repo.authenticate(user).is_ok() {
                let url = request.url();
                if url.contains("maven-metadata.xml") { put_metadata::handle(request, repo) } else { put_artifact::handle(request, repo) }
            } else {
                response_utils::unauthorised()
            }
        }
        None => response_utils::unauthorised(),
    }
}

pub fn handle_get(request: &Request, repo: RepoService) -> Response {
    let url = request.url();
    if url.ends_with("maven-metadata.xml") { get_metadata::handle(request, repo) } else { get_artifact::handle(request, repo) }
}

pub fn handle_head(request: &Request, repo: RepoService) -> Response {
    head_artifact::handle(request, repo)
}

pub fn handle_artifact_error(err: Error) -> Response {
    log::error!("{err:?}");
    match err {
        Error::InvalidFileArtifactUrl(_) => response_utils::not_found(),
        _ => response_utils::bad_request(),
    }
}

pub fn handle_error(err: Error) -> Response {
    log::error!("{err:?}");
    response_utils::bad_request()
}

pub fn extract_data_from_request(request: &Request) -> Result<Vec<u8>> {
    let mut body = request.data().ok_or_else(|| Error::RequestDataExtraction("Could not extract request body".to_string()))?;
    let mut buffer = Vec::new();
    body.read_to_end(&mut buffer).map_err(|e| Error::RequestDataExtraction(e.to_string()))?;
    Ok(buffer)
}
