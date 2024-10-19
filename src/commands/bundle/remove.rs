use crate::{CommandContext, SgResult};
use anyhow::Error;
use clap::{Parser, ValueEnum};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Parser)]
pub struct Remove {
    #[arg(required = true)]
    name: Vec<String>,

    #[arg(required = true, long, short, value_enum)]
    category: BundleCategory,

    #[arg(required = true, long, short)]
    group: String,

    #[arg(required = false, long)]
    skip_uninstall: bool,
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
enum BundleCategory {
    Bin,
    Cask,
}

impl Remove {
    pub async fn execute(&self, context: &CommandContext) -> SgResult<()> {
        let mut config = context.get_config();

        let bundle = config.bundle.get_or_insert(HashMap::new());

        if let Some(list) = bundle.get_mut(&self.group) {
            list.retain(|x| {
                let pkg = x.split(".").next().unwrap();
                !self.name.contains(&pkg.to_string())
            });
        } else {
            return Err(Error::msg(format!("Group {} not found", self.group)));
        }

        config.save();

        Ok(())
    }
}
