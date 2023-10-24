#![allow(unused)]
mod db;
mod process;

use clap::{Parser, Subcommand};
use std::{ffi::OsString, fmt::Error, io};

use db::Db;
use process::{Step, Process};

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
    List {},

}

fn generate_cli_steps(steps: usize) -> Vec<Step> {
    let mut array_steps: Vec<Step> = vec![];
    for n in 1..=steps {
        println!("Please provide the name of the step number {} -> ", n);
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        let step_name = name.trim_end().to_string();
        println!("Now provide a description -> ");
        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read input");
        let step_description = description.trim_end().to_string();
        let new_step: Step = Step {
            id: None,
            name: step_name,
            step_num: n,
            description: step_description,
        };
        array_steps.push(new_step);
    }

    array_steps
}

fn main() -> std::result::Result<(), rusqlite::Error> {
    let conn = Db::open()?;
    let args = Cli::parse();

    match args.command {
        Commands::New { name, steps } => {
            let generated_steps: Vec<Step> = generate_cli_steps(steps);
            let new_process: Process = Process {
                id: None,
                name,
                num_steps: steps,
                steps: generated_steps
            };
            conn.save_process(&new_process)?;
        }
        Commands::List {} => {
            conn.get_all()
                .into_iter()
                .for_each(|process| println!("{:#?}", process));
        }
    }
    Ok(())
}
