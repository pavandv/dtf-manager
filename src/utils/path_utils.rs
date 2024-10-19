use crate::consts::DTF_MANAGER_FILE;
use std::env;
use std::path::PathBuf;

use log::debug;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink;

pub struct PathDefaults {}
impl PathDefaults {
    pub const DOT_FILES: &'static str = "~/.dotfiles";
    pub const ZSH_CUSTOM: &'static str = "~/.dotfiles/custom";
}

pub struct PathUtils {}

impl PathUtils {
    pub fn get_path_buf(path: &str) -> PathBuf {
        if path.starts_with("~/") {
            let home = env::var_os("HOME").expect("HOME environment variable not set");
            let mut path_buf = PathBuf::from(home);
            path_buf.push(&path[2..]);
            path_buf
        } else {
            PathBuf::from(path)
        }
    }

    pub fn get_var(key: &str) -> String {
        match key {
            "DOT_FILES" => env::var(key).unwrap_or_else(|_| PathDefaults::DOT_FILES.to_string()),
            "ZSH_CUSTOM" => env::var(key).unwrap_or_else(|_| PathDefaults::ZSH_CUSTOM.to_string()),
            // Add more cases for other environment variables
            _ => env::var(key).unwrap_or_default(),
        }
    }

    pub fn get_dtf_file_path(file_name: &str) -> PathBuf {
        let dot_files_path = PathUtils::get_var("DOT_FILES");
        let path = format!("{}/{}", dot_files_path, file_name);
        PathUtils::get_path_buf(&path)
    }

    pub fn get_config_file_path() -> PathBuf {
        PathUtils::get_dtf_file_path(DTF_MANAGER_FILE)
    }

    pub fn create_symlink(source: &PathBuf, target: &PathBuf) -> std::io::Result<()> {
        debug!(
            "Creating symlink from {} to {}",
            source.display(),
            target.display()
        );

        symlink(source, target)
    }

    pub fn unexpand_home_dir(path: &PathBuf) -> PathBuf {
        if let Ok(home) = env::var("HOME") {
            if let Ok(stripped) = path.strip_prefix(&home) {
                return PathBuf::from("~").join(stripped);
            }
        }
        path.to_path_buf()
    }
}
