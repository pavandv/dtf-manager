use crate::config::Config;
use crate::utils::path_utils::PathUtils;
use anyhow::Error;
use log::info;
use std::path::PathBuf;

pub fn create_symlink(
    config: &mut Config,
    config_path: &PathBuf,
    remote_config_path: &PathBuf,
) -> Result<(), Error> {
    let result = PathUtils::create_symlink(remote_config_path, config_path);

    if result.is_ok() {
        info!("symlink created successfully");
        let remote = PathUtils::unexpand_home_dir(remote_config_path);
        let local = PathUtils::unexpand_home_dir(config_path);

        info!("Saving config");
        config.save_symlinks(&local, &remote);

        Ok(())
    } else {
        let msg = format!(
            "Failed to create symlink for config file: {}",
            result.err().unwrap()
        );
        Err(Error::msg(msg))
    }
}
