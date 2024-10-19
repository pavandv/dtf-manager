use crate::{enum_commands, Executor, SgResult};
use clap::{Parser, Subcommand};
use serde::Serialize;

mod create;
mod list;
mod load;

#[derive(Debug, Parser, Serialize)]
pub struct Ssh {
    #[clap(subcommand)]
    command: Commands,
}

enum_commands!(Ssh, (create, load, list));
