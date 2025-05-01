use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    declare_id,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_stake_pool::instruction::deposit_sol;
use stake_pool_wrapper_sdk::instruction::StakePoolWrapperInstruction;

declare_id!(env!("STAKE_POOL_WRAPPER_PROGRAM_ID"));

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = StakePoolWrapperInstruction::try_from_slice(instruction_data)?;

    match instruction {
        StakePoolWrapperInstruction::DepositSol(lamports_in) => {
            let account_info_iter = &mut accounts.iter();
            let stake_pool_info = next_account_info(account_info_iter)?;
            let withdraw_authority_info = next_account_info(account_info_iter)?;
            let reserve_stake_account_info = next_account_info(account_info_iter)?;
            let from_user_lamports_info = next_account_info(account_info_iter)?;
            let dest_user_pool_info = next_account_info(account_info_iter)?;
            let manager_fee_info = next_account_info(account_info_iter)?;
            let referrer_fee_info = next_account_info(account_info_iter)?;
            let pool_mint_info = next_account_info(account_info_iter)?;
            let system_program_info = next_account_info(account_info_iter)?;
            let token_program_info = next_account_info(account_info_iter)?;
            let stake_pool_program = next_account_info(account_info_iter)?;

            let deposit_ix = deposit_sol(
                stake_pool_program.key,
                stake_pool_info.key,
                withdraw_authority_info.key,
                reserve_stake_account_info.key,
                from_user_lamports_info.key,
                dest_user_pool_info.key,
                manager_fee_info.key,
                referrer_fee_info.key,
                pool_mint_info.key,
                token_program_info.key,
                lamports_in,
            );

            invoke(
                &deposit_ix,
                &[
                    stake_pool_info.clone(),
                    withdraw_authority_info.clone(),
                    reserve_stake_account_info.clone(),
                    from_user_lamports_info.clone(),
                    dest_user_pool_info.clone(),
                    manager_fee_info.clone(),
                    referrer_fee_info.clone(),
                    pool_mint_info.clone(),
                    system_program_info.clone(),
                    token_program_info.clone(),
                ],
            )?;
        }
    }

    Ok(())
}
