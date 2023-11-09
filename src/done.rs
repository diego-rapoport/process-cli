use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ToggleSub {
    /// Id of the process.
    #[command(subcommand)]
    pub done: ToggleCommands,
}

#[derive(Debug, Subcommand)]
pub enum ToggleCommands {
    Process {
        /// Id of the process.
        id: usize,
    },

    Step {
        /// Id of the step.
        id: usize,
    },
}
