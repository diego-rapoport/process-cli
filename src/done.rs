use clap::{Args, Subcommand};


#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DoneSub {
    /// Id of the process.
    #[command(subcommand)]
    pub done: DoneCommands,
}

#[derive(Debug, Subcommand)]
pub enum DoneCommands {

    Process {
        /// Id of the process.
        id: usize,

    },

    Step {
        /// Id of the step.
        id: usize,
    },
}


