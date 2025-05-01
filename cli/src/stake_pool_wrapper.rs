use std::path::PathBuf;

use clap::{command, Subcommand};
use solana_sdk::pubkey::Pubkey;

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
}
