use crate::consts::{DOT_FILES_PATH, ORIGINAL_ZSHRC_PATH, ORIGINAL_STARSHIP_PATH, ZINIT_INSTALL_URL};
use crate::SgResult;
use anyhow::Error;
use clap::Parser;
use duct::cmd;
use serde::Serialize;
use std::env;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink;

#[derive(Debug, Serialize, Parser)]
#[command(name = "init")]
pub struct Init {
    #[arg(long, short, required = true)]
    repo: String,
}

impl Init {
    pub async fn execute(&self) -> SgResult<()> {
        let zinit_home = env::var("zinit_home").unwrap_or_else(|_| "".to_string());

        let directory_path = PathBuf::from(zinit_home);

        if !directory_path.exists() && !directory_path.is_dir() {
            let cmd_str = format!(
                "curl --fail --show-error --silent --location {}",
                ZINIT_INSTALL_URL
            );

            let status = cmd!("sh", "-c", cmd_str).read();

            if status.is_err() {
                return Err(Error::msg(format!("Failed to install zinit")));
            }
        }

        let delete_files = [ORIGINAL_ZSHRC_PATH, ORIGINAL_STARSHIP_PATH];

        for file in delete_files {
            let path = PathBuf::from(file);
            if path.exists() {
                std::fs::remove_file(path).expect(format!("Failed to delete {}", file).as_str());
            }
        }
        
        let sym_link_files = [
            (format!("{}/{}", DOT_FILES_PATH, "zshrc"), ORIGINAL_ZSHRC_PATH),
            (format!("{}/{}", DOT_FILES_PATH, "starship.toml"), ORIGINAL_STARSHIP_PATH),
        ];
        
        for (sym_link_file, original_file) in sym_link_files {
            let symlink_path = PathBuf::from(sym_link_file);
            let original_path = PathBuf::from(original_file);

            match symlink(original_path, &symlink_path) {
                Ok(_) => println!("Symlink created successfully"),
                Err(e) => println!("Error creating symlink: {}", e),
            }
        }

        Ok(())
    }
}
