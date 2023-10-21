#![allow(unused)]
mod process;
mod db;

use std::{ffi::OsString, fmt::Result};
use clap::{Parser, Subcommand};

use crate::db::Db;

pub enum ProcessError {
    NumStepExists
}

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
#[clap(name = "processor")]
#[clap(about = "Create, view and mutate processes with multiple and changable steps.")]
#[clap(version = "1.0")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Create a new process with N number of steps. Provide the number of steps with the -s flag.
    New {
        /// Number of steps of this process.
        steps: usize,
    },

    /// List all of the processes already created.
    List {},

    #[clap(external_subcommand)]
    External(Vec<OsString>),
}

fn main() -> Result<()> {
    let db = Db::open()?;
    let args = Cli::parse();

    match args.command {
        Commands::New { steps } => todo!(),
        Commands::List {  } => {
            db.get_all_processes().unwrap().into_iter().for_each(|process| {
                println!("{:?}", process.id)
            });
        },
        Commands::External(_) => todo!(),
    }
    Ok(())

}
