use crate::SgResult;
use anyhow::Error;
use duct::cmd;
use log::{error, info};

// pub fn uninstall(pkg_name: &str) -> SgResult<()> {
//     println!();
//     info!("Installing {} with brew", pkg_name);
//     println!();
//     let result = cmd!("brew", "install", pkg_name).run();
//
//     if result.is_ok() {
//         println!();
//         info!("Successfully uninstalled.");
//         Ok(())
//     } else {
//         let msg = format!("Failed to uninstall {} with brew.", pkg_name);
//         println!();
//         error!("{}", msg);
//         Err(Error::msg(msg))
//     }
// }

pub fn install(pkg_name: &str) -> SgResult<()> {
    println!();
    info!("Installing {} with brew", pkg_name);
    println!();
    let result = cmd!("brew", "install", pkg_name).run();

    if result.is_ok() {
        println!();
        info!("Successfully installed.");
        Ok(())
    } else {
        println!();
        error!("Failed to install.");
        Err(Error::msg("Failed to install.".to_string()))
    }
}

pub fn install_cask(pkg_name: &str) -> SgResult<()> {
    println!();
    info!("Installing {} with brew cask", pkg_name);
    println!();

    let result = cmd!("brew", "install", "--cask", pkg_name).run();

    if result.is_ok() {
        println!();
        info!("Successfully installed cask.");
        Ok(())
    } else {
        println!();
        error!("Failed to install cask.");
        Err(Error::msg("Failed to install cask.".to_string()))
    }
}
