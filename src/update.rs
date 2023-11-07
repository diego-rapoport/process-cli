
use clap::{Args, Subcommand};


#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct UpdateSub {
    /// Id of the process.
    // #[arg(short, long, group = "type")]
    #[command(subcommand)]
    pub update: Option<UpdateCommands>,
}

#[derive(Debug, Subcommand)]
pub enum UpdateCommands {

    Process {
        /// Id of the process.
        id: usize,

        /// New name
        name: Option<String>,
    },

    Step {
        /// Id of the step.
        id: usize,

        /// New name.
        name: Option<String>,

        /// New description.
        description: Option<String>,
    },
}


