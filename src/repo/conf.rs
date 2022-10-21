use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Conf {
    pub repo_path: String,
}

impl Conf {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Conf> {
        let yaml = fs::read_to_string(&path)?;
        serde_yaml::from_str(&yaml)
            .map_err(|e|
                Error::InvalidConfiguration(format!("Error while parsing conf file({:?}) : {}", &path.as_ref().canonicalize(), e.to_string()))
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::repo::conf::Conf;

    #[test]
    fn test_import_config_file_ok() {
        let valid_path = "./config.yaml";
        let conf = Conf::from_file(valid_path);
        assert!(conf.is_ok());
        let conf = conf.unwrap();
        assert_eq!(conf.repo_path, "repository");
    }

    #[test]
    fn test_import_config_file_ko() {
        let invalid_path = "./config-invalid-path.yaml";
        let conf = Conf::from_file(invalid_path);
        assert!(conf.is_err());
    }
}
