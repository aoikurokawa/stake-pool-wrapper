use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum StakePoolWrapperInstruction {
    #[account(0, writable, name = "stake_pool")]
    #[account(1, name = "withdraw_authority")]
    #[account(2, writable, name = "reserve_stake")]
    #[account(3, signer, name = "from_user_lamports")]
    #[account(4, writable, name = "dest_user_pool")]
    #[account(5, writable, name = "manager_fee")]
    #[account(6, writable, name = "referrer_fee")]
    #[account(7, writable, name = "pool_mint")]
    #[account(8, name = "system_program")]
    #[account(9, name = "token_program")]
    #[account(10, name = "stake_pool_program")]
    DepositSol(u64),
}
