use std::process;
use crate::{commands::*, enum_commands, Executor, SgResult};
use clap::Parser;
use clap::Subcommand;
use serde::Serialize;

#[derive(Debug, Serialize, Parser)]
#[command(
    name = "dtf",
    about = "A CLI to manage Dot Files.",
    next_line_help = true
)]
pub struct DtfCli {
    #[clap(subcommand)]
    command: Commands,
}

enum_commands!(DtfCli, (init));

impl DtfCli {
    pub async fn run_from_args() -> SgResult<()> {
        let _cli = DtfCli::parse();

        _cli.run().await
    }

    pub async fn run(&self) -> SgResult<()> {
        let execute = self.execute().await;
        
        match execute {
            Ok(_) => {
                process::exit(0);
            }
            Err(err) => {
                eprintln!(
                    "{icon} {msg}",
                    icon = 'x'.to_string(),
                    msg = err.to_string().as_str()
                );
                process::exit(1);
            }
        }
    }
}
