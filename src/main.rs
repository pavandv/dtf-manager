use crate::dtf::DtfCli;
use anyhow::Error;

pub use command_context::CommandContext;
mod command_context;
pub mod commands;
mod config;
mod consts;
mod dtf;
mod installer;
mod macros;
mod services;
mod utils;

use macros::executor_trait::Executor;

pub type SgResult<T> = Result<T, Error>;

#[tokio::main]
async fn main() -> Result<SgResult<()>, std::io::Error> {
    Ok(DtfCli::run_from_args().await)
}
