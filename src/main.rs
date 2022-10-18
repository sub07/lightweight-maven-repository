mod xml;
mod maven;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

use log::{error, info, warn};
use quick_xml::se::to_string;
use rouille::{Request, Response};
use time::{format_description, Instant, OffsetDateTime};
use time::macros::format_description;
use crate::maven::extract_artifact_from_url;
use crate::xml::{ArtifactId, get_xml, GroupId, LastUpdated, Latest, Metadata, Release, Version, Versioning, Versions};

const REPO_DIR: &str = "repository";

fn print_request(request: &Request) {
    info!("{} - {}", request.method(), request.url());
    // info!("headers :");
    // for (key, value) in request.headers() {
    //     info!("{key} : {value}");
    // }
}


fn generate_maven_metadata_for_artifact(artifact_name: String, group_id: String) -> String {
    let path_str = format!("{REPO_DIR}/{}/{artifact_name}/", group_id.replace(".", "/"));
    let path = Path::new(&path_str);
    match path.read_dir() {
        Ok(dirs) => {
            let mut versions: Vec<Version> = dirs
                .filter(|dir| dir.is_ok())
                .map(|dir| dir.unwrap())
                .filter(|dir| dir.path().is_dir())
                .map(|dir| dir.file_name().into_string())
                .filter(|dir| dir.is_ok())
                .map(|dir| dir.unwrap())
                .map(|dir| Version { version: dir })
                .collect();

            versions.sort_by(|v1, v2| v1.version.cmp(&v2.version));

            let last_updated = {
                let last_updated_date_format = format_description::parse("[year][month][day][hour][minute][second]").unwrap();
                let now: OffsetDateTime = SystemTime::now().into();
                now.format(&last_updated_date_format).unwrap()
            };

            let versioning = Versioning {
                release: Release { body: versions.last().unwrap().version.to_string() },
                latest: Latest { body: versions.last().unwrap().version.to_string() },
                versions: Versions { version: versions },
                last_updated: LastUpdated { body: last_updated },
            };

            let metadata = Metadata {
                group_id: GroupId { body: group_id },
                artifact_id: ArtifactId { body: artifact_name },
                versioning,
            };

            let xml = get_xml(metadata);
            return xml;
            // let metadata_file_path = format!("{path_str}/maven-metadata.xml");
            // fs::write(metadata_file_path, xml).unwrap();
        }
        Err(err) => {
            error!("Could not read versions dir in artifact {artifact_name}");
            return "".to_string();
        }
    }
}

fn handle_get_maven_metadata(request: &Request) -> Response {
    let url = request.url();
    let url_parts = url.split("/").collect::<Vec<_>>();

    let artifact_name = url_parts[url_parts.len() - 2].to_string();
    let group_id = url_parts[1..url_parts.len() - 2].join(".");
    let maven_metadata = generate_maven_metadata_for_artifact(artifact_name, group_id);
    Response::text(maven_metadata)
}

fn handle_get(request: &Request) -> Response {
    if request.url().ends_with("maven-metadata.xml") { return handle_get_maven_metadata(request); }

    let path_str = format!("{REPO_DIR}{}", request.url());
    let path = Path::new(&path_str);

    let data = fs::read(path).unwrap();

    Response::from_data("application/octet-stream", data)
}
static BAD_RESPONSE: fn(&'static str) -> Response = |reason: &'static str| Response::text(reason).with_status_code(400);
static GOOD_RESPONSE: fn() -> Response = || Response::text("").with_status_code(200);

fn get_data_from_request(request: &Request) -> Result<Vec<u8>, String> {
    let data = match request.data() {
        None => {
            error!("Request {} - {} has no data", request.method(), request.url());
            return Err("No data".to_string());
        }
        Some(mut data) => {
            let mut buffer = Vec::new();
            match data.read_to_end(&mut buffer) {
                Ok(_) => buffer,
                Err(err) => {
                    error!("Could not load request body : {err}");
                    return Err("Could not read body".to_string());
                }
            }
        }
    };

    Ok(data)
}

fn handle_put_maven_metadata(path: &Path, request: &Request) -> Response {
    fs::remove_file(path);
    let data = get_data_from_request(request).unwrap();
    create_file_from_request(path, &data).unwrap();
    GOOD_RESPONSE()
}

fn create_file_from_request(path: &Path, data: &[u8]) -> Result<(), String> {
    match fs::create_dir_all(path.parent().expect("Path should be a file so calling parent() should not fail")) {
        Ok(()) => {}
        Err(err) => {
            error!("Could not create dir for file {:?} : {err}", path.canonicalize());
            return Err("Could not create dir for file".to_string());
        }
    }

    match fs::write(path, data) {
        Ok(()) => {}
        Err(err) => {
            error!("Could not write file {:?} : {err}", path.canonicalize());
            return Err("Could not create file".to_string());
        }
    }

    Ok(())
}

fn handle_put(request: &Request) -> Response {
    let path_str = format!("{REPO_DIR}{}", request.url());
    let path = Path::new(&path_str);

    if request.url().contains("maven-metadata.xml") { return handle_put_maven_metadata(&path, request); }

    if path.is_file() {
        error!("File {:?} already exists", path.canonicalize());
        return BAD_RESPONSE("This artifact version already exists");
    }
    if let None = request.header("Content-Type") {
        error!("Request {} - {} has no content-type", request.method(), request.url());
        return BAD_RESPONSE("Request has no content-type");
    }
    if let Some(v) = request.header("Content-Type") {
        if v != "application/octet-stream" {
            error!("Request {} - {} has wrong content-type : {v}", request.method(), request.url());
            return BAD_RESPONSE("Request has wrong content-type");
        }
    }

    let data = get_data_from_request(request).unwrap();

    match create_file_from_request(&path, &data) {
        Ok(_) => {}
        Err(err) => {
            error!("Could not write file {:?} : {err}", path.canonicalize());
            return BAD_RESPONSE("Could not put file");
        }
    };

    GOOD_RESPONSE()
}

fn handle_head(request: &Request) -> Response {
    let path_str = format!("{REPO_DIR}{}", request.url());
    let path = Path::new(&path_str);

    let file_len = match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(err) => {
            warn!("Could not get size of {path_str}");
            0
        }
    };

    Response::text("")
        .with_unique_header("Content-Length", file_len.to_string())
}

fn transform_response(response: Response) -> Response {
    let response = response.with_unique_header("Server", "Lightweight Maven Repository Server");
    response
}

fn main() -> std::io::Result<()> {
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    rouille::start_server("127.0.0.1:3000", move |request| {
        print_request(request);

        let response = match request.method() {
            // "POST" => handle_post(request),
            "GET" => handle_get(request),
            "PUT" => handle_put(request),
            "HEAD" => handle_head(request),
            _ => Response::text("").with_status_code(403)
        };

        transform_response(response)
    });
}
