use std::str::FromStr;

use borsh::BorshDeserialize;
use borsh_legacy::BorshSerialize;
use solana_program::stake;
use solana_sdk::{
    borsh1::try_from_slice_unchecked,
    instruction::{AccountMeta, Instruction},
    native_token::{sol_to_lamports, LAMPORTS_PER_SOL},
    pubkey::Pubkey,
    signer::Signer,
    system_program, sysvar,
};
use spl_associated_token_account::get_associated_token_address;
use spl_stake_pool::{
    instruction::{
        add_validator_to_pool_with_vote, deposit_sol, update_stake_pool_balance, withdraw_sol,
        StakePoolInstruction,
    },
    state::{AccountType, StakePool, ValidatorList, ValidatorListHeader},
};
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
                action: StakePoolWrapperActions::WithdrawSol { amount },
            } => self.withdraw_sol(amount),
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::UpdateStakePoolBalance,
            } => self.update_stake_pool_balance(),
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::AddValidator,
            } => self.add_validator(),
            StakePoolWrapperCommands::StakePoolWrapper {
                action: StakePoolWrapperActions::IncreaseValidatorStake,
            } => self.increase_validator_stake(),
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

        let rpc_client = self.get_rpc_client();
        let acc_data = rpc_client.get_account_data(&stake_pool).unwrap();
        println!("Account Data: {acc_data:?}");

        let stake_pool = try_from_slice_unchecked::<StakePool>(&acc_data).unwrap();
        println!("Stake Pool: {stake_pool:?}");

        // let account_type = AccountType::StakePool;
        // println!("Account Type: {account_type:?}");

        // let deposit_sol_ix = borsh::to_vec(&StakePoolInstruction::DepositSol(100)).unwrap();
        // println!("Instruction: {:?}", deposit_sol_ix);

        Ok(())
    }

    /// Deposit SOL
    fn deposit_sol(&self, amount: u64) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let admin = signer.pubkey();

        let stake_pool = Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        // let reserve_stake_pool =
        //     Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        let reserve_stake_pool =
            Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();

        let depositer = admin;
        let user = Pubkey::from_str("3XKzfD4NT6Qk1sU9iYnEKKAf8AU8D3YurW1MgjF8bomc").unwrap();
        let fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        // let fee = Pubkey::from_str("9DuzpRvid1HQX8nRwznmu35uUtb92euBNcL12Dk1qTY8").unwrap();
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
    fn fail_deposit_sol(&self, amount: u64) -> anyhow::Result<()> {
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
    fn wrapper_deposit_sol(&self, amount: u64) -> anyhow::Result<()> {
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

    /// Withdraw SOL
    fn withdraw_sol(&self, amount: u64) -> anyhow::Result<()> {
        let signer = self.signer()?;
        let admin = signer.pubkey();

        let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        let stake_pool = Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        let user_transfer_authority = admin;
        let pool_tokens_from = get_associated_token_address(&user_transfer_authority, &pool_mint);
        let reserve_stake_pool =
            Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        let fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        let _system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let token_program =
            Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let stake_pool_program =
            Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        let ix = withdraw_sol(
            &stake_pool_program,
            &stake_pool,
            &stake_pool_withdraw_authority,
            &user_transfer_authority,
            &pool_tokens_from,
            &reserve_stake_pool,
            &user_transfer_authority,
            &fee,
            &pool_mint,
            &token_program,
            amount * LAMPORTS_PER_SOL,
        );

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

    fn add_validator(&self) -> anyhow::Result<()> {
        let signer = self.signer()?;
        // let admin = signer.pubkey();
        let stake_pool_program =
            Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        let stake_pool_pubkey =
            Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        // let reserve_stake_pool =
        //     Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();
        // let reserve_stake_pool =
        //     Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();

        // let depositer = admin;
        // let user = Pubkey::from_str("3XKzfD4NT6Qk1sU9iYnEKKAf8AU8D3YurW1MgjF8bomc").unwrap();
        // let fee = Pubkey::from_str("DH7tmjoQ5zjqcgfYJU22JqmXhP5EY1tkbYpgVWUS2oNo").unwrap();
        // let fee = Pubkey::from_str("9DuzpRvid1HQX8nRwznmu35uUtb92euBNcL12Dk1qTY8").unwrap();
        // let referral_fee =
        //     Pubkey::from_str("22Mjmaea25LDrpEQyJonfV6ybkrDcxGDsoCqUH39Cw9m").unwrap();
        // let pool_mint = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();
        // let _system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        // let token_program =
        //     Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        // let stake_pool_program =
        //     Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        let validator = Pubkey::from_str("vgcDar2pryHvMgPkKaZfh8pQy4BJxv7SpwUG7zinWjG").unwrap();

        let staker = Pubkey::from_str("aaaDerwdMyzNkoX1aSoTi3UtFe2W45vh5wCgQNhsjF8").unwrap();

        let validator_list_pubkey =
            Pubkey::from_str("nZ5vUrsJjHcvkJsfKP1b1RgSEZUMJmwFpk7NksTeX5A").unwrap();

        // let client = self.get_rpc_client();
        // let validator_list_acc = client.get_account(&validator_list_pubkey).unwrap();

        // let mut validator_list_acc_data = validator_list_acc.data;
        // let (header, validator_list) =
        //     ValidatorListHeader::deserialize_vec(&mut validator_list_acc_data).unwrap();

        // let stake_pool_acc = client.get_account(&stake_pool_pubkey).unwrap();
        // let mut stake_pool_acc_data = stake_pool_acc.data;
        // let stake_pool: StakePool = try_from_slice_unchecked(&mut stake_pool_acc_data).unwrap();

        // let validator_list: ValidatorList =
        //     try_from_slice_unchecked(&validator_list_acc.data).unwrap();

        // println!("Validator List: {header:?}");

        let stake = Pubkey::find_program_address(
            &[&validator.to_bytes(), &stake_pool_pubkey.to_bytes()],
            &stake_pool_program,
        )
        .0;
        // let (stake_account_address, _) = find_stake_program_address(
        //     &solana_program::pubkey::Pubkey::from_str(
        //         "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy",
        //     )
        //     .unwrap(),
        //     &solana_program::pubkey::Pubkey::from_str(
        //         "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy",
        //     )
        //     .unwrap(),
        //     stake_pool_address,
        // );
        // let stake = Pubkey::from_str("nZ5vUrsJjHcvkJsfKP1b1RgSEZUMJmwFpk7NksTeX5B").unwrap();

        let ix = add_validator_to_pool(
            &stake_pool_program,
            &stake_pool_pubkey,
            &staker,
            &signer.pubkey(),
            &stake_pool_withdraw_authority,
            &validator_list_pubkey,
            &stake,
            &validator,
        );

        // let ix = add_validator_to_pool_with_vote(
        //     &stake_pool_program,
        //     &stake_pool,
        //     &stake_pool_pubkey,
        //     &validator,
        //     None,
        // );
        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        Ok(())
    }

    fn increase_validator_stake(&self) -> anyhow::Result<()> {
        let signer = self.signer()?;
        // let admin = signer.pubkey();
        let stake_pool_program =
            Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

        let stake_pool_pubkey =
            Pubkey::from_str("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb").unwrap();
        let stake_pool_withdraw_authority =
            Pubkey::from_str("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS").unwrap();
        let reserve_stake_pool =
            Pubkey::from_str("rrWBQqRqBXYZw3CmPCCcjFxQ2Ds4JFJd7oRQJ997dhz").unwrap();

        let validator = Pubkey::from_str("5a7ETzLjwYdWkJNUiGqoGrw4rYeEuDyy9aqiwJSbGRat").unwrap();

        let staker = Pubkey::from_str("BBBATax9kikSHQp8UTcyQL3tfU3BmQD9yid5qhC7QEAA").unwrap();

        let validator_list_pubkey =
            Pubkey::from_str("nZ5vUrsJjHcvkJsfKP1b1RgSEZUMJmwFpk7NksTeX5A").unwrap();

        let client = self.get_rpc_client();
        let validator_list_acc = client.get_account(&validator_list_pubkey).unwrap();

        let validator_list: ValidatorList =
            try_from_slice_unchecked(&validator_list_acc.data).unwrap();
        let validator_stake_info = validator_list
            .find(&validator)
            .ok_or("Vote account not found in validator list")
            .unwrap();

        let seed: u64 = validator_stake_info.transient_seed_suffix.into();
        let transient_stake = Pubkey::find_program_address(
            &[
                b"transient",
                &validator.to_bytes(),
                &stake_pool_pubkey.to_bytes(),
                &seed.to_le_bytes(),
            ],
            &stake_pool_program,
        )
        .0;
        let ix = increase_validator_stake(
            &stake_pool_program,
            &stake_pool_pubkey,
            &staker,
            &stake_pool_withdraw_authority,
            &validator_list_pubkey,
            &reserve_stake_pool,
            &transient_stake,
            &validator,
            sol_to_lamports(1.0),
            seed,
        );

        // let ix = add_validator_to_pool_with_vote(
        //     &stake_pool_program,
        //     &stake_pool,
        //     &stake_pool_pubkey,
        //     &validator,
        //     None,
        // );
        let ixs = [ix];
        self.process_transaction(&ixs, &signer.pubkey(), &[signer])?;

        Ok(())
    }
}

fn add_validator_to_pool(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    staker: &Pubkey,
    reserve: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    validator_list: &Pubkey,
    stake: &Pubkey,
    validator: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new_readonly(*staker, true),
        AccountMeta::new(*reserve, false),
        AccountMeta::new_readonly(*stake_pool_withdraw, false),
        AccountMeta::new(*validator_list, false),
        AccountMeta::new(*stake, false),
        AccountMeta::new_readonly(*validator, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::stake_history::id(), false),
        #[allow(deprecated)]
        AccountMeta::new_readonly(stake::config::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(stake::program::id(), false),
    ];
    // let data = borsh::to_vec(&StakePoolInstruction::AddValidatorToPool( seed.map(|s| s.get()).unwrap_or(0),
    // ))
    // .unwrap();
    Instruction {
        program_id: *program_id,
        accounts,
        data: spl_stake_pool_legacy::instruction::StakePoolInstruction::AddValidatorToPool
            .try_to_vec()
            .unwrap(),
    }
}

pub fn increase_validator_stake(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    staker: &Pubkey,
    stake_pool_withdraw_authority: &Pubkey,
    validator_list: &Pubkey,
    reserve_stake: &Pubkey,
    transient_stake: &Pubkey,
    validator: &Pubkey,
    lamports: u64,
    transient_stake_seed: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*stake_pool, false),
        AccountMeta::new_readonly(*staker, true),
        AccountMeta::new_readonly(*stake_pool_withdraw_authority, false),
        AccountMeta::new(*validator_list, false),
        AccountMeta::new(*reserve_stake, false),
        AccountMeta::new(*transient_stake, false),
        AccountMeta::new_readonly(*validator, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::stake_history::id(), false),
        AccountMeta::new_readonly(stake::config::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(stake::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: spl_stake_pool_legacy::instruction::StakePoolInstruction::IncreaseValidatorStake {
            lamports,
            transient_stake_seed,
        }
        .try_to_vec()
        .unwrap(),
    }
}
