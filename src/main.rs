#![allow(unused)]
mod db;
mod process;
mod step;
mod parsed;
mod ui;

use clap::{Parser, Subcommand};
use std::{ffi::OsString, fmt::Error, io};

use db::Db;
use process::Process;
use step::Step;

pub enum ProcessError {
    NumStepExists,
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

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new process with N number of steps.
    New {
        /// Name of the process.
        name: String,
        /// Number of steps of this process.
        steps: usize,
    },

    /// List all of the processes already created.
    List { id: Option<usize> },

    /// List steps from a particular process. Query by id.
    Steps {id: usize},

    /// Update a process or step with the respective id.
    #[group(required = true, multiple = false)]
    Update {
        #[arg(short, long)]
        process: Option<usize>,

        #[arg(short, long)]
        step: Option<usize>,
    },
}

fn main() -> std::result::Result<(), rusqlite::Error> {
    let conn = Db::open()?;
    let args = Cli::parse();

    match args.command {
        Commands::New { name, steps } => {
            let generated_steps: Vec<Step> = ui::generate_cli_steps(steps);
            let new_process: Process = Process {
                id: None,
                name,
                num_steps: steps,
                steps: generated_steps,
                is_finished: false,
            };
            conn.save_process(&new_process)?;
        }

        Commands::List { id } => {
            match id {
                Some(expr) => {
                    conn.get_processes_from_id(id.unwrap())
                        .into_iter()
                        .for_each(|process| println!("{:#?}", process));
                }
                None => {
                    conn.get_all()
                        .into_iter()
                        .for_each(|process| println!("{:#?}", process));
                }
            }
        }

        Commands::Steps { id } => {
            conn.get_steps_from_process(id).into_iter().for_each(|step| println!("{:#?}", step))
        }

        Commands::Update { process, step } => {
            if process.is_some(){
                println!("Process id = {:?}", process);
            }
            if step.is_some() {
                println!("Step id = {:?}", step);

            }
        },
    }
    Ok(())
}
