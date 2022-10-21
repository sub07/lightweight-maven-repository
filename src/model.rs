#[derive(Debug)]
pub struct Artifact {
    pub group_id: String,
    pub name: String,
    pub version: Option<String>,
}
