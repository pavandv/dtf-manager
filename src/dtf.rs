use clap::Parser;
use clap::Subcommand;
use env_logger::Builder;
use log::LevelFilter;
use serde::Serialize;
use std::process;

use crate::{commands::*, enum_commands, CommandContext, Executor, SgResult};

#[derive(Debug, Serialize, Parser)]
#[command(
    name = "dtf",
    about = "A CLI to manage Dot Files.",
    next_line_help = true
)]
pub struct DtfCli {
    #[clap(subcommand)]
    command: Commands,

    // #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,
}

enum_commands!(DtfCli, (bundle, init, ssh, symlink));

impl DtfCli {
    pub async fn run_from_args() -> SgResult<()> {
        let _cli = DtfCli::parse();

        let log_level = match _cli.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        Builder::new().filter_level(log_level).init();

        _cli.run().await
    }

    pub async fn run(&self) -> SgResult<()> {
        let context = CommandContext::new()?;
        let execute = self.execute(&context).await;

        match execute {
            Ok(_) => {
                let config = context.get_config();
                config.save();
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
