#![allow(unused)]
mod db;
mod process;
mod step;
mod parsed;
mod ui;

use clap::{Parser, Subcommand, Args};
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

#[derive(Subcommand, Debug )]
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

    /// Update a process or step with the respective id
    Update(UpdateSub),
}

#[derive(Debug, Args)]
struct UpdateSub {
        /// Id of the process.
        #[arg(short, long, group = "type")]
        process: Option<usize>,

        /// Id of the step.
        #[arg(short, long, group = "type")]
        step: Option<usize>,

        /// Name of the process/step to update.
        #[arg(short, long)]
        name: Option<String>,

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
                Some(id) => {
                    conn.get_processes_from_id(id)
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

        Commands::Update(update) => {
            match update {
                process => {
                    conn.update_process_name(process.process.unwrap(), process.name.unwrap())
                },
                step => {
                    println!("Its a step")
                }
            }
        },

    }
    Ok(())
}
