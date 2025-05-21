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
    DepositSol { amount: u64 },

    /// Fail deposit SOL
    FailDepositSol { amount: u64 },

    /// Deposit SOL
    WrapperDepositSol { amount: u64 },

    /// Update Stake Pool Balance
    UpdateStakePoolBalance,
}
