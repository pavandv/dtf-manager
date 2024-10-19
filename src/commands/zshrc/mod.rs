use crate::{enum_commands, Executor, SgResult};
use clap::{Parser, Subcommand};
use serde::Serialize;

mod generate;

#[derive(Debug, Serialize, Parser)]
pub struct Zshrc {
    #[clap(subcommand)]
    command: Commands,
}

enum_commands!(Zshrc, (generate));
