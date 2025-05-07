use clap::{command, Subcommand};

#[derive(Subcommand)]
pub enum StakePoolWrapperCommands {
    StakePoolWrapper {
        #[command(subcommand)]
        action: StakePoolWrapperActions,
    },
}

/// Stake Pool Wrapper commands
#[derive(Subcommand)]
pub enum StakePoolWrapperActions {
    /// Deposit SOL
    DepositSol,

    /// Deposit SOL
    WrapperDepositSol,

    /// Update Stake Pool Balance
    UpdateStakePoolBalance,
}
