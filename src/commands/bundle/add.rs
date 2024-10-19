use crate::installer::brew;
use crate::{CommandContext, SgResult};
use clap::{Parser, ValueEnum};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Parser)]
pub struct Add {
    #[arg(required = true)]
    name: Vec<String>,

    #[arg(required = true, long, short, value_enum)]
    category: BundleCategory,

    #[arg(required = true, long, short)]
    group: String,

    #[arg(required = false, long)]
    skip_install: bool,
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
enum BundleCategory {
    Bin,
    Cask,
}

// implement display for BundleCategory
impl std::fmt::Display for BundleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleCategory::Bin => write!(f, "bin"),
            BundleCategory::Cask => write!(f, "cask"),
        }
    }
}

impl Add {
    pub async fn execute(&self, context: &CommandContext) -> SgResult<()> {
        let mut config = context.get_config();

        let bundle = config.bundle.get_or_insert(HashMap::new());

        let group = bundle.entry(self.group.clone()).or_insert_with(Vec::new);

        let command = match self.category {
            BundleCategory::Bin => brew::install,
            BundleCategory::Cask => brew::install_cask,
        };

        for name in self.name.clone() {
            if !self.skip_install {
                command(&name)?;
            }
            group.push(format!("{}.{}", name, self.category.to_string()));
        }

        config.save();

        Ok(())
    }
}
