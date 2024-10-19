use crate::consts::SSH_DIR;
use crate::utils::path_utils::PathUtils;
use crate::{CommandContext, SgResult};
use clap::Parser;
use duct::cmd;
use log::info;
use serde::Serialize;

#[derive(Debug, Parser, Serialize)]
pub struct Create {
    #[arg(required = true, long)]
    email: String,

    #[arg(required = true, long, short)]
    name: String,
}

impl Create {
    pub async fn execute(&self, _: &CommandContext) -> SgResult<()> {
        let ssh_dir = PathUtils::get_path_buf(SSH_DIR);

        info!("Checking if ssh directory exists");
        if !ssh_dir.exists() {
            info!("Creating ssh directory");
            // create the ssh directory using tokio
            tokio::fs::create_dir_all(&ssh_dir).await?;
        }

        info!("Creating {} ssh key", self.name);
        cmd!(
            "ssh-keygen",
            "-t",
            "rsa",
            "-b",
            "4096",
            "-C",
            &self.email,
            "-o",
            "-f",
            &self.name
        )
        .run()?;
        info!("Successfully created ssh key");

        Ok(())
    }
}
