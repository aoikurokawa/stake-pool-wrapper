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
    /// Get
    Get,

    /// Deposit SOL
    DepositSol { amount: u64 },

    /// Fail deposit SOL
    FailDepositSol { amount: u64 },

    /// Deposit SOL
    WrapperDepositSol { amount: u64 },

    /// Withdraw SOL
    WithdrawSol { amount: u64 },

    /// Update Stake Pool Balance
    UpdateStakePoolBalance,
}
