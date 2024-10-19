use crate::{enum_commands, Executor, SgResult};
use clap::{Parser, Subcommand};
use serde::Serialize;

mod create;

#[derive(Debug, Parser, Serialize)]
pub struct Symlink {
    #[clap(subcommand)]
    command: Commands,
}

enum_commands!(Symlink, (create));
