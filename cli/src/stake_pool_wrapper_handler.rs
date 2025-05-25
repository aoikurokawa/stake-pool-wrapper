use std::str::FromStr;

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signer::Signer,
};
use spl_stake_pool::instruction::{deposit_sol, update_stake_pool_balance, StakePoolInstruction};
use stake_pool_wrapper_sdk::instruction::StakePoolWrapperInstruction;

use crate::{
    cli_config::CliConfig,
    stake_pool_wrapper::{StakePoolWrapperActions, StakePoolWrapperCommands},
    CliHandler,
};

pub struct StakePoolWrapperCliHandler {
    /// The configuration of CLI
    cli_config: CliConfig,
}

impl CliHandler for StakePoolWrapperCliHandler {
    fn cli_config(&self) -> &CliConfig {
        &self.cli_config
    }
}

/// Handle Vault Whitelist
impl StakePoolWrapperCliHandler {
    pub const fn new(cli_config: CliConfig) -> Self {
        Self { cli_config }
    }

    pub fn handle(&self, action: StakePoolWrapperCommands) -> anyhow::Result<()> {
        match action {
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::Get,
            } => self.get(),
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::DepositSol { amount },
            } => self.deposit_sol(amount),
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::FailDepositSol { amount },
            } => self.fail_deposit_sol(amount),
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::WrapperDepositSol { amount },
            } => self.wrapper_deposit_sol(amount),
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::UpdateStakePoolBalance,
            } => self.update_stake_pool_balance(),
        }
    }
}

/// Handle Stake Pool Wrapper
impl StakePoolWrapperCliHandler {
    /// Deposit SOL
    pub fn get(&self) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let _admin = signer.pubkey();

        let stake_pool = Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        // let stake_pool_withdraw_authority =
        //     Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        // let reserve_stake_pool =
        //     Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        // let depositer = admin;
        // let user = Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        // let fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        // let referral_fee =
        //     Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        // let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        // let _system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        // let token_program =
        //     Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        // let stake_pool_program =
        //     Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        // let ix = deposit_sol(
        //     &stake_pool_program,
        //     &stake_pool,
        //     &stake_pool_withdraw_authority,
        //     &reserve_stake_pool,
        //     &depositer,
        //     &user,
        //     &fee,
        //     &referral_fee,
        //     &pool_mint,
        //     &token_program,
        //     amount * LAMPORTS_PER_SOL,
        // );

        // let ixs = [ix];
        // self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        let rpc_client = self.get_rpc_client();
        let acc_data = rpc_client.get_account_data(&stake_pool).unwrap();

        println!("Account Data: {acc_data:?}");

        let deposit_sol_ix = borsh::to_vec(&StakePoolInstruction::DepositSol(100)).unwrap();
        println!("Instruction: {:?}", deposit_sol_ix);

        Ok(())
    }

    /// Deposit SOL
    pub fn deposit_sol(&self, amount: u64) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let admin = signer.pubkey();

        let stake_pool = Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        let reserve_stake_pool =
            Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        let depositer = admin;
        let user = Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        let fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        let referral_fee =
            Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        let _system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let token_program =
            Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let stake_pool_program =
            Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        let ix = deposit_sol(
            &stake_pool_program,
            &stake_pool,
            &stake_pool_withdraw_authority,
            &reserve_stake_pool,
            &depositer,
            &user,
            &fee,
            &referral_fee,
            &pool_mint,
            &token_program,
            amount * LAMPORTS_PER_SOL,
        );

        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        Ok(())
    }

    /// Fail Depositing SOL
    pub fn fail_deposit_sol(&self, amount: u64) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let admin = signer.pubkey();

        let stake_pool = Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("2V6Abua9BY6Ga8HUeLWSLXh4Gm6oKsn3GpTzP4eYMFqT").unwrap();
        let reserve_stake_pool =
            Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        let depositer = admin;
        let user = Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        let fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        let referral_fee =
            Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        let _system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let token_program =
            Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let stake_pool_program =
            Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        let ix = deposit_sol(
            &stake_pool_program,
            &stake_pool,
            &stake_pool_withdraw_authority,
            &reserve_stake_pool,
            &depositer,
            &user,
            &fee,
            &referral_fee,
            &pool_mint,
            &token_program,
            amount * LAMPORTS_PER_SOL,
        );

        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        Ok(())
    }

    /// Wrapper Deposit SOL
    pub fn wrapper_deposit_sol(&self, amount: u64) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let admin = signer.pubkey();

        let stake_pool = Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        let reserve_stake_pool =
            Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        let depositer = admin;
        let user = Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        let fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        let referral_fee =
            Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        let system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let token_program =
            Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let stake_pool_program =
            Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        let accounts = vec![
            AccountMeta::new(stake_pool, false),
            AccountMeta::new_readonly(stake_pool_withdraw_authority, false),
            AccountMeta::new(reserve_stake_pool, false),
            AccountMeta::new(depositer, true),
            AccountMeta::new(user, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(referral_fee, false),
            AccountMeta::new(pool_mint, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(stake_pool_program, false),
        ];

        let ix = Instruction {
            program_id: Pubkey::from_str("BVeDMMWtnCUD2mXyyoXQ6bHnA2vrTvnavisnFZXAsRPr").unwrap(),
            accounts,
            data: borsh::to_vec(&StakePoolWrapperInstruction::DepositSol(
                amount * LAMPORTS_PER_SOL,
            ))
            .unwrap(),
        };

        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        Ok(())
    }

    /// Deposit SOL
    pub fn update_stake_pool_balance(&self) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let _admin = signer.pubkey();

        let stake_pool = Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        let validator_list =
            Pubkey::from_str("nZ5vUrsJjHcvkJsfKP1b1RgSEZUMJmwFpk7NksTeX5A").unwrap();
        let reserve_stake =
            Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        let manager_fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        let _referral_fee =
            Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        let _system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let token_program =
            Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let stake_pool_program =
            Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        // let accounts = vec![
        //     AccountMeta::new(stake_pool, false),
        //     AccountMeta::new_readonly(stake_pool_withdraw_authority, false),
        //     AccountMeta::new(reserve_stake_pool, false),
        //     AccountMeta::new(depositer, true),
        //     AccountMeta::new(user, false),
        //     AccountMeta::new(fee, false),
        //     AccountMeta::new(referral_fee, false),
        //     AccountMeta::new(pool_mint, false),
        //     AccountMeta::new_readonly(system_program, false),
        //     AccountMeta::new_readonly(token_program, false),
        //     AccountMeta::new_readonly(stake_pool_program, false),
        // ];

        let ix = update_stake_pool_balance(
            &stake_pool_program,
            &stake_pool,
            &stake_pool_withdraw_authority,
            &validator_list,
            &reserve_stake,
            &manager_fee,
            &pool_mint,
            &token_program,
        );

        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        Ok(())
    }
}
