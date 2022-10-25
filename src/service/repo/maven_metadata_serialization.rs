use std::cmp::Ordering;

use quick_xml::se::to_string;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "metadata")]
pub struct Metadata {
    #[serde(rename = "groupId")]
    pub group_id: GroupId,
    #[serde(rename = "artifactId")]
    pub artifact_id: ArtifactId,
    pub versioning: Versioning,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct GroupId {
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ArtifactId {
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Versioning {
    pub latest: Latest,
    pub release: Release,
    pub versions: Versions,
    #[serde(rename = "lastUpdated")]
    pub last_updated: LastUpdated,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Versions {
    pub version: Vec<Version>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Latest {
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Release {
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct LastUpdated {
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Version {
    #[serde(rename = "$value")]
    pub version: String,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        String::partial_cmp(&self.version, &other.version)
    }

    fn lt(&self, other: &Self) -> bool {
        String::lt(&self.version, &other.version)
    }

    fn le(&self, other: &Self) -> bool {
        String::le(&self.version, &other.version)
    }

    fn gt(&self, other: &Self) -> bool {
        String::gt(&self.version, &other.version)
    }

    fn ge(&self, other: &Self) -> bool {
        String::ge(&self.version, &other.version)
    }
}

pub fn serialize_xml(metadata: Metadata) -> String {
    let xml = to_string(&metadata).expect("should work");
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>{xml}"#)
}
