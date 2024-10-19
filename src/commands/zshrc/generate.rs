use crate::command_context::CommandContext;
use crate::consts::REMOTE_ZSHRC_PATH;
use crate::utils::path_utils::PathUtils;
use crate::SgResult;
use clap::Parser;
use log::debug;
use serde::Serialize;
use std::env;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Parser)]
pub struct Generate {}

impl Generate {
    pub async fn execute(&self, _: &CommandContext) -> SgResult<()> {
        self.create_variables().await?;
        self.create_hist_overrides().await?;

        let zshrc_path = PathUtils::get_path_buf(REMOTE_ZSHRC_PATH);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(zshrc_path)
            .await?;

        file.write_all(
            r#"
        source "$HOME/$DOT_FILES/variables.sh"
        source "$HOME/$DOT_FILES/hist_overrides.sh"
        "#
            .as_bytes(),
        )
        .await?;

        debug!("{}", env::current_exe().ok().unwrap().display());
        Ok(())
    }

    async fn create_variables(&self) -> SgResult<()> {
        let variables_file_name = "variables.sh";
        let dot_files_path = PathUtils::get_var("DOT_FILES");

        let variables_path = format!("{}/{}", dot_files_path, variables_file_name);
        let variables_path_buf = PathUtils::get_path_buf(&variables_path);

        if !variables_path_buf.exists() {
            let mut file = File::create(&variables_path).await?;

            file.write_all(include_bytes!("../../../templates/variables"))
                .await?;
        };

        Ok(())
    }

    async fn create_hist_overrides(&self) -> SgResult<()> {
        let file_name = "hist_overrides.sh";
        let dot_files_path = PathUtils::get_var("DOT_FILES");

        let path = format!("{}/{}", dot_files_path, file_name);
        let path_buf = PathUtils::get_path_buf(&path);

        if !path_buf.exists() {
            let mut file = File::create(&path).await?;

            file.write_all(include_bytes!("../../../templates/hist_overrides"))
                .await?;
        };

        Ok(())
    }
}
