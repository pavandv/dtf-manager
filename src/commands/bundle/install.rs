use crate::installer::brew;
use crate::{CommandContext, SgResult};
use anyhow::Error;
use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(Debug, Serialize, Parser)]
pub struct Install {
    #[arg(required = true)]
    group: String,
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
enum BundleCategory {
    Bin,
    Cask,
}

impl Install {
    pub async fn execute(&self, context: &CommandContext) -> SgResult<()> {
        let config = context.get_config();

        if let Some(bundle) = &config.bundle {
            let group = bundle
                .get(&self.group)
                .ok_or_else(|| Error::msg(format!("Bundle group {} not found", self.group)))?;

            for name in group {
                let (name, command) = name.split_once('.').unwrap();
                let result = match command {
                    "bin" => brew::install(name),
                    "cask" => brew::install_cask(name),
                    _ => Err(Error::msg(format!("Unknown command {}", command))),
                };

                if let Err(err) = result {
                    println!("{}", err);
                }
            }
        } else {
            return Err(Error::msg("No bundle group specified"));
        }

        Ok(())
    }
}
