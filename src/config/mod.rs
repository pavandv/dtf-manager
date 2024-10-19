use crate::utils::path_utils::PathUtils;
use figment::{
    providers::{Format, YamlExtended},
    Figment,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bundle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<HashMap<String, Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub casks: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symlinks: Option<Vec<Vec<PathBuf>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<HashMap<String, Vec<String>>>,
}

impl Config {
    pub fn figment() -> Figment {
        let config_path = PathUtils::get_config_file_path();
        Figment::new().join(YamlExtended::file(config_path))
    }

    pub fn reload(&mut self) {
        *self = Self::figment().extract::<Self>().unwrap();
    }

    pub fn save(&self) {
        let config_path = PathUtils::get_config_file_path();
        debug!("Saving config to {}", config_path.display());

        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(config_path);

        if let Ok(mut file) = file {
            let yaml = serde_yaml::to_string(self).unwrap();
            debug!("{}", yaml);
            file.write_all(yaml.as_bytes()).unwrap();
        }
    }

    pub fn save_symlinks(&mut self, source: &PathBuf, target: &PathBuf) {
        let mut new_symlinks = match &self.symlinks {
            Some(symlinks) => symlinks.clone(),
            None => vec![],
        };

        let is_exists = new_symlinks.iter().any(|link| link[0].eq(source));

        if !is_exists {
            new_symlinks.push(vec![source.clone(), target.clone()]);

            self.symlinks = Some(new_symlinks);
            // self.save();
        }
    }

    pub fn reset_config(&mut self) {
        *self = Config {
            version: None,
            symlinks: None,
            bundle: None,
        };
    }
}
