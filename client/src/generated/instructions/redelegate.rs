//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
#[derive(Debug)]
pub struct Redelegate {
  }

impl Redelegate {
  pub fn instruction(&self, args: RedelegateInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: RedelegateInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity( remaining_accounts.len());
        accounts.extend_from_slice(remaining_accounts);
    let mut data = borsh::to_vec(&RedelegateInstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&args).unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::SPL_STAKE_POOL_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct RedelegateInstructionData {
            discriminator: u8,
                              }

impl RedelegateInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: 22,
                                                                          }
  }
}

impl Default for RedelegateInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct RedelegateInstructionArgs {
                  pub lamports: u64,
                pub source_transient_stake_seed: u64,
                pub ephemeral_stake_seed: u64,
                pub destination_transient_stake_seed: u64,
      }


/// Instruction builder for `Redelegate`.
///
/// ### Accounts:
///
#[derive(Clone, Debug, Default)]
pub struct RedelegateBuilder {
                    lamports: Option<u64>,
                source_transient_stake_seed: Option<u64>,
                ephemeral_stake_seed: Option<u64>,
                destination_transient_stake_seed: Option<u64>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RedelegateBuilder {
  pub fn new() -> Self {
    Self::default()
  }
                    #[inline(always)]
      pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.lamports = Some(lamports);
        self
      }
                #[inline(always)]
      pub fn source_transient_stake_seed(&mut self, source_transient_stake_seed: u64) -> &mut Self {
        self.source_transient_stake_seed = Some(source_transient_stake_seed);
        self
      }
                #[inline(always)]
      pub fn ephemeral_stake_seed(&mut self, ephemeral_stake_seed: u64) -> &mut Self {
        self.ephemeral_stake_seed = Some(ephemeral_stake_seed);
        self
      }
                #[inline(always)]
      pub fn destination_transient_stake_seed(&mut self, destination_transient_stake_seed: u64) -> &mut Self {
        self.destination_transient_stake_seed = Some(destination_transient_stake_seed);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = Redelegate {
            };
          let args = RedelegateInstructionArgs {
                                                              lamports: self.lamports.clone().expect("lamports is not set"),
                                                                  source_transient_stake_seed: self.source_transient_stake_seed.clone().expect("source_transient_stake_seed is not set"),
                                                                  ephemeral_stake_seed: self.ephemeral_stake_seed.clone().expect("ephemeral_stake_seed is not set"),
                                                                  destination_transient_stake_seed: self.destination_transient_stake_seed.clone().expect("destination_transient_stake_seed is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}


/// `redelegate` CPI instruction.
pub struct RedelegateCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
        /// The arguments for the instruction.
    pub __args: RedelegateInstructionArgs,
  }

impl<'a, 'b> RedelegateCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
              args: RedelegateInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity( remaining_accounts.len());
        remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = borsh::to_vec(&RedelegateInstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&self.__args).unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::SPL_STAKE_POOL_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(1 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
        remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `Redelegate` via CPI.
///
/// ### Accounts:
///
#[derive(Clone, Debug)]
pub struct RedelegateCpiBuilder<'a, 'b> {
  instruction: Box<RedelegateCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RedelegateCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(RedelegateCpiBuilderInstruction {
      __program: program,
                                            lamports: None,
                                source_transient_stake_seed: None,
                                ephemeral_stake_seed: None,
                                destination_transient_stake_seed: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
                    #[inline(always)]
      pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.instruction.lamports = Some(lamports);
        self
      }
                #[inline(always)]
      pub fn source_transient_stake_seed(&mut self, source_transient_stake_seed: u64) -> &mut Self {
        self.instruction.source_transient_stake_seed = Some(source_transient_stake_seed);
        self
      }
                #[inline(always)]
      pub fn ephemeral_stake_seed(&mut self, ephemeral_stake_seed: u64) -> &mut Self {
        self.instruction.ephemeral_stake_seed = Some(ephemeral_stake_seed);
        self
      }
                #[inline(always)]
      pub fn destination_transient_stake_seed(&mut self, destination_transient_stake_seed: u64) -> &mut Self {
        self.instruction.destination_transient_stake_seed = Some(destination_transient_stake_seed);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = RedelegateInstructionArgs {
                                                              lamports: self.instruction.lamports.clone().expect("lamports is not set"),
                                                                  source_transient_stake_seed: self.instruction.source_transient_stake_seed.clone().expect("source_transient_stake_seed is not set"),
                                                                  ephemeral_stake_seed: self.instruction.ephemeral_stake_seed.clone().expect("ephemeral_stake_seed is not set"),
                                                                  destination_transient_stake_seed: self.instruction.destination_transient_stake_seed.clone().expect("destination_transient_stake_seed is not set"),
                                    };
        let instruction = RedelegateCpi {
        __program: self.instruction.__program,
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct RedelegateCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
                    lamports: Option<u64>,
                source_transient_stake_seed: Option<u64>,
                ephemeral_stake_seed: Option<u64>,
                destination_transient_stake_seed: Option<u64>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

