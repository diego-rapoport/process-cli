#![allow(unused)]
mod db;
mod parsed;
mod process;
mod step;
mod ui;
mod update;
mod done;

use clap::{Args, Parser, Subcommand};
use done::{DoneSub, DoneCommands};
use std::{ffi::OsString, fmt::Error, io};

use db::Db;
use process::Process;
use step::Step;
use update::{UpdateSub, UpdateCommands};

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
    Steps { id: usize },

    /// Update a process or step with the respective id.
    Update(UpdateSub),

    /// Toggle as done/undone a process or just a step.
    Done(DoneSub),
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

        Commands::List { id } => match id {
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
        },

        Commands::Steps { id } => conn
            .get_steps_from_process(id)
            .into_iter()
            .for_each(|step| println!("{:#?}", step)),

        Commands::Update(update) => {
            let update_cmd = update.update.unwrap();
            match update_cmd {
                UpdateCommands::Process { id, name } => match name {
                    Some(name) => {
                        conn.update_process_name(id, name);
                        return Ok(());
                    }
                    None => return Ok(()),
                },
                UpdateCommands::Step {
                    id,
                    name,
                    description,
                } => {
                    if (name.is_some()) {
                        conn.update_step_name(id, name.unwrap());
                    }
                    if (description.is_some()) {
                        conn.update_step_description(id, description.unwrap())
                    }
                }
            }
        }

        Commands::Done(done) => {
            let done_cmd = done.done.unwrap();
            match done_cmd {
                DoneCommands::Process { id } => todo!(),
                DoneCommands::Step { id } => todo!(),
            }
        },
    }
    Ok(())
}
