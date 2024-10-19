use crate::{enum_commands, Executor, SgResult};
use clap::{Parser, Subcommand};
use serde::Serialize;

mod add;
mod install;
mod remove;

#[derive(Debug, Parser, Serialize)]
pub struct Bundle {
    #[clap(subcommand)]
    command: Commands,
}

enum_commands!(Bundle, (add, remove, install));
