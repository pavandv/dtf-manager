use crate::{CommandContext, SgResult};
use clap::Parser;
use duct::cmd;
use log::info;
use serde::Serialize;

#[derive(Debug, Parser, Serialize)]
pub struct List;

impl List {
    pub async fn execute(&self, _: &CommandContext) -> SgResult<()> {
        info!("fetching all ssh profiles added to the agent");
        cmd!("ssh-add", "-l").run()?;

        Ok(())
    }
}
