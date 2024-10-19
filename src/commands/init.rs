use crate::{
    command_context::CommandContext,
    config::Config,
    consts::{
        HAMMERSPOON_CONFIG_DIR, HAMMERSPOON_CONFIG_PATH, LOCAL_STARSHIP_PATH, LOCAL_ZSHRC_PATH,
    },
    installer::brew,
    services::{pkg_manager, restore::restore, symlink},
    utils::{file::FileUtils, path_utils::PathUtils},
    SgResult,
};

use anyhow::Error;
use clap::{Parser, ValueEnum};
use duct::cmd;
use log::{debug, error, info};
use serde::Serialize;
use std::{fs, fs::File, io::Write, ops::Deref};

#[derive(Debug, Serialize, Parser)]
#[command(name = "init")]
pub struct Init {
    /// The repository to clone
    #[arg(long)]
    repo: Option<String>,

    #[arg(long, value_enum, default_value = "antigen")]
    pkg: String,
}

#[derive(Debug, Serialize, Clone, ValueEnum)]
enum Pkg {
    Antigen,
    OMZ,
}

// implement display for Pkg
impl std::fmt::Display for Pkg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pkg::Antigen => write!(f, "antigen"),
            Pkg::OMZ => write!(f, "omz"),
        }
    }
}

impl Init {
    pub async fn execute(&self, context: &CommandContext) -> SgResult<()> {
        let mut config = context.get_config();

        let local_zshrc_file = PathUtils::get_path_buf(LOCAL_ZSHRC_PATH);
        let remote_zshrc_path = PathUtils::get_dtf_file_path(".zshrc");

        let dot_files_path = PathUtils::get_var("DOT_FILES");
        let dot_files_path_buf = PathUtils::get_path_buf(&dot_files_path);

        self.install_xcode_tools().await?;

        debug!("dot_files_path_buf: {}", dot_files_path_buf.display());
        if dot_files_path_buf.exists() {
            info!("Deleting existing Dot files directory");
            tokio::fs::remove_dir_all(&dot_files_path_buf).await?;
            config.reset_config();
        }

        if let Some(repo) = &self.repo {
            info!(
                "Cloning DotFiles repository '{}' to '{}'",
                repo,
                dot_files_path_buf.display()
            );
            let result = cmd!("git", "clone", repo, &dot_files_path_buf).run()?;

            if result.status.success() {
                info!("Repository cloned successfully");
            } else {
                error!("Failed to clone repository");
                return Err(Error::msg("Failed to clone repository".to_string()));
            }
        } else {
            info!("Creating DotFiles directory: {}", dot_files_path);
            fs::create_dir(&dot_files_path_buf).expect("Failed to create dot_files directory");
        }

        self.install_pkg_manager(&context).await?;
        self.install_packages(&mut config).await?;

        debug!("is dtf managed: {}", self.is_dtf_managed());

        if self.is_dtf_managed() {
            info!("DotFiles is already managed by DTF");
            config.reload();
            restore(config.deref()).await?;
        } else {
            let variables_path = PathUtils::get_dtf_file_path("variables.sh");
            File::create(&variables_path).expect("Failed to create config file");

            FileUtils::prepend_to_file(
                &local_zshrc_file,
                format!("source {}/variables.sh\n", dot_files_path),
            )
            .await?;

            fs::rename(&local_zshrc_file, &remote_zshrc_path)
                .expect("Failed to move local zshrc file");

            // File::create(&remote_zshrc_path).expect("Failed to create zshrc file");
            symlink::create_symlink(&mut config, &local_zshrc_file, &remote_zshrc_path)?;

            config.version = Some("0.1.0".to_string());
        }

        Ok(())
    }

    async fn install_pkg_manager(&self, context: &CommandContext) -> SgResult<()> {
        match self.pkg.as_str() {
            "antigen" => pkg_manager::install_antigen(&context).await,
            "omz" => pkg_manager::install_omz(&context).await,
            _ => {
                error!("Invalid package manager specified");
                Err(Error::msg("Invalid package manager specified".to_string()))
            }
        }
    }

    fn is_dtf_managed(&self) -> bool {
        PathUtils::get_config_file_path().exists()
    }

    async fn install_xcode_tools(&self) -> SgResult<()> {
        let xcode_tools_path = PathUtils::get_path_buf("/Library/Developer/CommandLineTools");
        if xcode_tools_path.exists() {
            info!("Xcode tools already installed");
            return Ok(());
        }

        info!("Installing Xcode tools");
        let result = cmd!("xcode-select", "--install").run();

        if result.is_ok() {
            info!("Xcode tools installed successfully");
            Ok(())
        } else {
            error!("Failed to install Xcode tools");
            Err(Error::msg("Failed to install Xcode tools".to_string()))
        }
    }

    async fn install_packages(&self, config: &mut Config) -> Result<(), Error> {
        // Install starship
        self.install_starship(config).await?;

        // Install HammerSpoon
        self.install_hammerspoon(config).await?;

        Ok(())
    }

    async fn install_starship(&self, config: &mut Config) -> Result<(), Error> {
        brew::install("starship")?;

        if !self.is_dtf_managed() {
            let config_path = PathUtils::get_path_buf(LOCAL_STARSHIP_PATH);
            let remote_config_path = PathUtils::get_dtf_file_path(".starship.toml");

            File::create(&remote_config_path).expect("Failed to create config file");

            let result = symlink::create_symlink(config, &config_path, &remote_config_path);

            if result.is_ok() {
                info!("Starship config installed successfully");
                let zshrc_path = PathUtils::get_path_buf(LOCAL_ZSHRC_PATH);
                FileUtils::write(
                    &zshrc_path,
                    "\n\neval \"$(starship init zsh)\"\n".to_string(),
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn install_hammerspoon(&self, config: &mut Config) -> Result<(), Error> {
        brew::install_cask("hammerspoon")?;
        let hammerspoon_config_dir = PathUtils::get_path_buf(HAMMERSPOON_CONFIG_DIR);
        let hammerspoon_config_path = PathUtils::get_path_buf(HAMMERSPOON_CONFIG_PATH);

        let hammerspoon_spoons_dir = hammerspoon_config_dir.join("Spoons");
        let remote_hammerspoon_config_path = PathUtils::get_dtf_file_path(".hammerspoon-init.lua");

        // create the hammerspoon config file along with the directory
        fs::create_dir_all(&hammerspoon_spoons_dir)
            .expect("Failed to create hammerspoon config directory");

        info!("Downloading hammerspoon SpoonInstall spoon");
        let spoon_install_url =
            "https://github.com/Hammerspoon/Spoons/raw/master/Spoons/SpoonInstall.spoon.zip";
        debug!("Downloading spoon: {}", spoon_install_url);

        let response = reqwest::get(spoon_install_url).await?;
        debug!("Response status: {}", response.status());
        if response.status().is_success() {
            let bytes = response.bytes().await?;

            debug!("Creating temp file in {}", hammerspoon_spoons_dir.display());
            let temp_file_path = hammerspoon_spoons_dir.join("temp.zip");
            let mut temp_file = File::create(&temp_file_path)?;
            debug!("Writing to temp file: {}", temp_file_path.display());
            let result = temp_file.write_all(&bytes);
            let flush_result = temp_file.flush();

            if result.is_ok() && flush_result.is_ok() {
                info!("Downloaded spoon");
                let zip_file =
                    zip::ZipArchive::new(File::open(hammerspoon_spoons_dir.join("temp.zip"))?);

                info!("Extracting spoon");
                let zs = zip_file?.extract(&hammerspoon_spoons_dir);

                if zs.is_ok() {
                    info!("Extracted spoon successfully");
                    tokio::fs::remove_file(&temp_file_path).await?;
                }
            }

            if !self.is_dtf_managed() {
                tokio::fs::File::create(&remote_hammerspoon_config_path).await?;

                symlink::create_symlink(
                    config,
                    &hammerspoon_config_path,
                    &remote_hammerspoon_config_path,
                )?;
            }

            Ok(())
        } else {
            error!("Failed to download spoon");
            Err(Error::msg("Failed to download spoon"))
        }
    }
}
