use crate::config::Config;
use crate::installer::brew;
use crate::utils::path_utils::PathUtils;
use crate::SgResult;
use log::{debug, info};

pub async fn restore(config: &Config) -> SgResult<()> {
    info!("Restoring configuration");

    let sym_links = config.symlinks.as_ref().unwrap();

    let zshrc = PathUtils::get_path_buf("~/.zshrc");

    if zshrc.exists() {
        info!("Removing existing .zshrc");
        std::fs::remove_file(&zshrc)?;
    }

    info!("Restoring symlinks");
    for sym_link in sym_links {
        let source = PathUtils::get_path_buf(sym_link[0].to_str().unwrap());
        let target = PathUtils::get_path_buf(sym_link[1].to_str().unwrap());

        if !source.is_symlink() {
            info!(
                "Creating symlink: {} -> {}",
                source.display(),
                target.display()
            );
            PathUtils::create_symlink(&target, &source)?;
        } else {
            info!(
                "Symlink already exists: {} -> {}",
                source.display(),
                target.display()
            );
        }
    }

    info!("Installing default bundle");
    let bundles = config.bundle.as_ref();

    if let Some(bundles) = bundles {
        debug!("bundles: {:?}", bundles);

        let default_bundle = bundles.get("default");

        if let Some(default_bundle) = default_bundle {
            debug!("default bundle: {:?}", default_bundle);

            for brew_pkg in default_bundle {
                debug!("brew_pkg: {}", brew_pkg);
                let parts: Vec<&str> = brew_pkg.split('.').collect();
                if let [pkg, command] = parts.as_slice() {
                    if command.to_string() == "cask" {
                        info!("Installing cask: {}", pkg);
                        brew::install_cask(pkg)?;
                    } else {
                        info!("Installing package: {}", pkg);
                        brew::install(pkg)?;
                    }
                }
            }
        } else {
            info!("No default bundle found");
        }
    } else {
        debug!("No bundles found");
    }

    Ok(())
}
