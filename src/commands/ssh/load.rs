use crate::consts::SSH_DIR;
use crate::utils::path_utils::PathUtils;
use crate::{CommandContext, SgResult};
use anyhow::Error;
use clap::Parser;
use duct::cmd;
use log::{debug, info};
use serde::Serialize;

#[derive(Debug, Parser, Serialize)]
pub struct Load {
    #[arg(required = true)]
    name: String,
}

impl Load {
    pub async fn execute(&self, _: &CommandContext) -> SgResult<()> {
        let ssh_dir = PathUtils::get_path_buf(SSH_DIR);

        let key = ssh_dir.join(&self.name);

        if !key.exists() {
            let msg = format!("ssh key '{}' does not exist", key.display());
            return Err(Error::msg(msg));
        }

        debug!("unloading all ssh profiles");
        cmd!("ssh-add", "-D").run()?;

        info!("Adding {} ssh key to the agent", self.name);
        cmd!("ssh-add", &key).run()?;

        Ok(())
    }
}
