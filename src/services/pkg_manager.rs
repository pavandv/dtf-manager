use crate::consts::{LOCAL_ZSHRC_PATH, OMZ_INSTALL_URL};
use crate::installer::brew::install;
use crate::utils::path_utils::PathUtils;
use crate::{CommandContext, SgResult};
use anyhow::Error;
use duct::{cmd, IntoExecutablePath};
use log::{debug, info};
use std::{env, fs};
use tokio::fs::File;

pub async fn install_omz(context: &CommandContext) -> SgResult<()> {
    let zsh_path = env::var_os("ZSH").unwrap_or_else(|| "".to_executable());
    let path_buf = PathUtils::get_path_buf(zsh_path.to_str().unwrap());

    let zsh_custom_path = PathUtils::get_var("ZSH_CUSTOM");
    let zsh_custom_path_buf = PathUtils::get_path_buf(&zsh_custom_path);

    debug!("ZSH Home value: {:?}", zsh_path);
    info!("Checking if 'Oh My Zsh' is installed");

    if !path_buf.exists() {
        info!("Oh My Zsh is not installed, installing...");
        let cmd = format!("curl -fsSL {} | bash", OMZ_INSTALL_URL);
        let cmd_str = format!("$({})", cmd);
        info!("Downloading OMZ install script from {}", OMZ_INSTALL_URL);
        info!("Installing OMZ");

        debug!("Executing command: {}", cmd_str);

        let status = cmd!("bash", "-c", cmd).env("NO_INPUT", "1").run();

        if status.is_ok() {
            info!("OMZ installed successfully");

            if !context.is_dtf_managed() {
                debug!("Creating ZSH Custom directory");
                fs::create_dir_all(&zsh_custom_path_buf)
                    .expect("Failed to create ZSH Custom directory");
            }

            Ok(())
        } else {
            Err(Error::msg("Failed to install OMZ".to_string()))
        }
    } else {
        info!("OMZ is already installed");
        Ok(())
    }
}

pub async fn install_antigen(context: &CommandContext) -> SgResult<()> {
    info!("Installing Antigen via brew");
    install("antigen")?;

    if !context.is_dtf_managed() {
        debug!("Creating .zshrc file");
        let zsh_custom_path_buf = PathUtils::get_path_buf(LOCAL_ZSHRC_PATH);

        File::create(&zsh_custom_path_buf).await?;
    }

    Ok(())
}
