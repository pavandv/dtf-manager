use crate::dtf::DtfCli;
use anyhow::Error;

pub mod commands;
mod consts;
mod dtf;
mod macros;

use macros::executor_trait::Executor;

pub type SgResult<T> = Result<T, Error>;

#[tokio::main]
async fn main() -> Result<SgResult<()>, std::io::Error> {
    Ok(DtfCli::run_from_args().await)
}
