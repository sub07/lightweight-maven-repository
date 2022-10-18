mod xml;

use std::fs;
use std::io::Read;
use std::path::Path;

use log::{error, info};
use quick_xml::se::to_string;
use rouille::{Request, Response};
use crate::xml::{ArtifactId, get_xml, GroupId, LastUpdated, Latest, Metadata, Release, Version, Versioning, Versions};

const REPO_DIR: &str = "repository";

fn print_request(request: &Request) {
    info!("{} - {}", request.method(), request.url());
    // info!("headers :");
    // for (key, value) in request.headers() {
    //     info!("{key} : {value}");
    // }
}

fn handle_get(request: &Request) -> Response {
    Response::text("GET response")
}

fn handle_put(request: &Request) -> Response {
    let path_str = format!("{REPO_DIR}{}", request.url());
    let path = Path::new(&path_str);
    let bad_response = |reason: &'static str| Response::text(reason).with_status_code(400);
    let good_response = Response::text("").with_status_code(200);

    if path.is_file() {
        error!("File {:?} already exists", path.canonicalize());
        return bad_response("This artifact version already exists");
    }
    if let None = request.header("Content-Type") {
        error!("Request {} - {} has no content-type", request.method(), request.url());
        return bad_response("Request has no content-type");
    }
    if let Some(v) = request.header("Content-Type") {
        if v != "application/octet-stream" {
            error!("Request {} - {} has wrong content-type : {v}", request.method(), request.url());
            return bad_response("Request has wrong content-type");
        }
    }

    let data = match request.data() {
        None => {
            error!("Request {} - {} has no data", request.method(), request.url());
            return bad_response("No data");
        }
        Some(mut data) => {
            let mut buffer = Vec::new();
            match data.read_to_end(&mut buffer) {
                Ok(_) => buffer,
                Err(err) => {
                    error!("Could not load request body : {err}");
                    return bad_response("Could not read body");
                }
            }
        }
    };

    match fs::create_dir_all(path.parent().expect("Path should be a file so calling parent() should not fail")) {
        Ok(()) => {}
        Err(err) => {
            error!("Could not create dir for file {:?} : {err}", path.canonicalize());
            return bad_response("Could not put file");
        }
    }

    match fs::write(path, data) {
        Ok(()) => {}
        Err(err) => {
            error!("Could not write file {:?} : {err}", path.canonicalize());
            return bad_response("Could not put file");
        }
    }

    good_response
}

fn transform_response(response: Response) -> Response {
    let response = response.with_unique_header("Server", "Lightweight Maven Repository Server");
    response
}

fn main() -> std::io::Result<()> {
    // if let Err(_e) = std::env::var("RUST_LOG") {
    //     std::env::set_var("RUST_LOG", "info");
    // }
    // pretty_env_logger::init();
    //
    // rouille::start_server("127.0.0.1:3000", move |request| {
    //     print_request(request);
    //
    //     let response = match request.method() {
    //         // "POST" => handle_post(request),
    //         "GET" => handle_get(request),
    //         "PUT" => handle_put(request),
    //         _ => Response::text("").with_status_code(403)
    //     };
    //
    //     transform_response(response)
    // });

    let version = vec![
        Version { version: String::from("1.0") },
        Version { version: String::from("1.0") },
        Version { version: String::from("1.0") },
        Version { version: String::from("1.0") },
        Version { version: String::from("1.0") },
        Version { version: String::from("1.0") },
        Version { version: String::from("1.0") },
        Version { version: String::from("1.0") },
    ];

    let versioning = Versioning {
        release: Release { body: "1.1".to_string() },
        latest: Latest { body: "1.1".to_string() },
        versions: Versions { version },
        last_updated: LastUpdated { body: "20221017162743".to_string() },
    };

    let metadata = Metadata {
        group_id: GroupId { body: "dev.mpardo".to_string() },
        artifact_id: ArtifactId { body: "angine".to_string() },
        versioning,
    };

    let xml = get_xml(metadata);
    fs::write("test.xml", &xml).unwrap();
    println!("{xml}");
    Ok(())
}
