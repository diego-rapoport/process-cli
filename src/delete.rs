use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DeleteSub {
    /// Id of the process.
    #[command(subcommand)]
    pub delete: DeleteCommands,
}

#[derive(Debug, Subcommand)]
pub enum DeleteCommands {
    Process {
        /// Id of the process.
        id: usize,
    },

    Step {
        /// Id of the step.
        id: usize,
    },
}
