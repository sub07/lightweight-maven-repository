use std::fs;
use std::path::Path;
use std::time::SystemTime;

use time::{format_description, OffsetDateTime};

use crate::error::{Error, Result};
use crate::service::conf::{Conf, User};
use crate::service::repo::maven_metadata_serialization::{ArtifactId, GroupId, LastUpdated, Latest, Metadata, Release, serialize_xml, Version, Versioning, Versions};

mod maven_metadata_serialization;

#[derive(Clone, Debug)]
pub struct RepoService {
    conf: Conf,
}

impl RepoService {
    pub fn new() -> Result<RepoService> {
        let config_file_path_env = "CONFIG_FILE_PATH";
        let config_file_path = std::env::var(config_file_path_env).unwrap_or_else(|_| "config.yaml".to_string());
        Ok(RepoService {
            conf: Conf::from_file(config_file_path)?,
        })
    }

    pub fn authenticate(&self, user: User) -> Result<()> {
        if self.conf.users.contains(&user) { Ok(()) } else { Err(Error::AuthenticationError) }
    }

    pub fn write_artifact<D: AsRef<[u8]>>(&self, artifact_url: String, data: D) -> Result<()> {
        self.write(artifact_url, data, false)
    }

    pub fn write_maven_metadata<D: AsRef<[u8]>>(&self, metadata_url: String, data: D) -> Result<()> {
        self.write(metadata_url, data, true)
    }

    fn write<D: AsRef<[u8]>>(&self, url: String, data: D, allow_overwrite: bool) -> Result<()> {
        let path_str = self.build_path(&url);
        let path = Path::new(&path_str);

        if path.extension().is_none() { return Err(Error::InvalidFileMetadataUrl(url)); }
        if path.exists() {
            if allow_overwrite { fs::remove_file(path)?; } else { return Err(Error::ArtifactOverwrite(url)); }
        } else { fs::create_dir_all(path.parent().expect("Path should be a file so calling parent() should not fail"))?; }

        fs::write(path, data)?;

        Ok(())
    }

    pub fn read(&self, url: String) -> Result<Vec<u8>> {
        let path_str = self.build_path(&url);
        let path = Path::new(&path_str);

        Ok(fs::read(path)?)
    }

    pub fn get_artifact_size(&self, url: String) -> Result<usize> {
        let path_str = self.build_path(&url);
        let path = Path::new(&path_str);

        match fs::metadata(path) {
            Ok(metadata) => Ok(metadata.len() as usize),
            Err(_) => Err(Error::InvalidFileArtifactUrl(url)),
        }
    }

    fn build_path(&self, url: &str) -> String {
        format!("{}/{}", self.conf.repo_path, url)
    }

    pub fn generate_maven_metadata(&self, url: String) -> Result<String> {
        let url_parts = url.split("/").collect::<Vec<_>>();

        let artifact_name = url_parts[url_parts.len() - 2].to_string();
        let group_id = url_parts[1..url_parts.len() - 2].join(".");

        let path_str = format!("{}/{}/{artifact_name}/", self.conf.repo_path, group_id.replace(".", "/"));
        let path = Path::new(&path_str);
        return match path.read_dir() {
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

                Ok(serialize_xml(metadata))
            }
            Err(err) => {
                Err(Error::MavenMetadataCreation(err.to_string()))
            }
        };
    }
}
