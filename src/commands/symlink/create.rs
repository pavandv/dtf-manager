use crate::services::symlink::create_symlink;
use crate::utils::path_utils::PathUtils;
use crate::{CommandContext, SgResult};
use clap::Parser;
use log::info;
use serde::Serialize;

#[derive(Debug, Parser, Serialize)]
pub struct Create {
    #[arg(required = true)]
    source: String,

    #[arg(required = true)]
    destination: String,
}

impl Create {
    pub async fn execute(&self, context: &CommandContext) -> SgResult<()> {
        let source_path = PathUtils::get_path_buf(&self.source);
        let destination_path = PathUtils::get_dtf_file_path(&self.destination);

        info!(
            "Creating symlink from {} to {}",
            source_path.display(),
            destination_path.display()
        );

        if source_path.exists() {
            return Err(anyhow::Error::msg(format!(
                "Source path {} exist, symlink can't be created",
                source_path.display()
            )));
        }

        if !destination_path.exists() {
            tokio::fs::create_dir_all(&destination_path.parent().unwrap()).await?;
        }

        let mut config = context.get_config();

        create_symlink(&mut config, &source_path, &destination_path)?;

        Ok(())
    }
}
