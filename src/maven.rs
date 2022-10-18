pub struct Artifact {
    pub group_id: String,
    pub artifact_name: String,
    pub version: String,
}

pub fn extract_artifact_from_url(url: String) -> Artifact {
    let url_parts = url.split("/").collect::<Vec<_>>();

    let version = url_parts[url_parts.len() - 2].to_string();
    let artifact_name = url_parts[url_parts.len() - 3].to_string();
    let group_id = url_parts[1..url_parts.len() - 3].join(".");

    Artifact {
        group_id,
        artifact_name,
        version,
    }
}
