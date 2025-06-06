/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/kit';
import { SPL_STAKE_POOL_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const WITHDRAW_SOL_DISCRIMINATOR = 16;

export function getWithdrawSolDiscriminatorBytes() {
  return getU8Encoder().encode(WITHDRAW_SOL_DISCRIMINATOR);
}

export type WithdrawSolInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountStakePool extends string | IAccountMeta<string> = string,
  TAccountWithdrawAuthority extends string | IAccountMeta<string> = string,
  TAccountUserTransferAuthority extends string | IAccountMeta<string> = string,
  TAccountBurnFromPool extends string | IAccountMeta<string> = string,
  TAccountReserveStake extends string | IAccountMeta<string> = string,
  TAccountDestinationLamports extends string | IAccountMeta<string> = string,
  TAccountManagerFee extends string | IAccountMeta<string> = string,
  TAccountPoolMint extends string | IAccountMeta<string> = string,
  TAccountClock extends string | IAccountMeta<string> = string,
  TAccountStakeHistory extends string | IAccountMeta<string> = string,
  TAccountStakeProgram extends string | IAccountMeta<string> = string,
  TAccountTokenProgram extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountStakePool extends string
        ? WritableAccount<TAccountStakePool>
        : TAccountStakePool,
      TAccountWithdrawAuthority extends string
        ? ReadonlyAccount<TAccountWithdrawAuthority>
        : TAccountWithdrawAuthority,
      TAccountUserTransferAuthority extends string
        ? ReadonlySignerAccount<TAccountUserTransferAuthority> &
            IAccountSignerMeta<TAccountUserTransferAuthority>
        : TAccountUserTransferAuthority,
      TAccountBurnFromPool extends string
        ? WritableAccount<TAccountBurnFromPool>
        : TAccountBurnFromPool,
      TAccountReserveStake extends string
        ? WritableAccount<TAccountReserveStake>
        : TAccountReserveStake,
      TAccountDestinationLamports extends string
        ? WritableAccount<TAccountDestinationLamports>
        : TAccountDestinationLamports,
      TAccountManagerFee extends string
        ? WritableAccount<TAccountManagerFee>
        : TAccountManagerFee,
      TAccountPoolMint extends string
        ? WritableAccount<TAccountPoolMint>
        : TAccountPoolMint,
      TAccountClock extends string
        ? ReadonlyAccount<TAccountClock>
        : TAccountClock,
      TAccountStakeHistory extends string
        ? ReadonlyAccount<TAccountStakeHistory>
        : TAccountStakeHistory,
      TAccountStakeProgram extends string
        ? ReadonlyAccount<TAccountStakeProgram>
        : TAccountStakeProgram,
      TAccountTokenProgram extends string
        ? ReadonlyAccount<TAccountTokenProgram>
        : TAccountTokenProgram,
      ...TRemainingAccounts,
    ]
  >;

export type WithdrawSolInstructionData = {
  discriminator: number;
  args: bigint;
};

export type WithdrawSolInstructionDataArgs = { args: number | bigint };

export function getWithdrawSolInstructionDataEncoder(): Encoder<WithdrawSolInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['args', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: WITHDRAW_SOL_DISCRIMINATOR })
  );
}

export function getWithdrawSolInstructionDataDecoder(): Decoder<WithdrawSolInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['args', getU64Decoder()],
  ]);
}

export function getWithdrawSolInstructionDataCodec(): Codec<
  WithdrawSolInstructionDataArgs,
  WithdrawSolInstructionData
> {
  return combineCodec(
    getWithdrawSolInstructionDataEncoder(),
    getWithdrawSolInstructionDataDecoder()
  );
}

export type WithdrawSolInput<
  TAccountStakePool extends string = string,
  TAccountWithdrawAuthority extends string = string,
  TAccountUserTransferAuthority extends string = string,
  TAccountBurnFromPool extends string = string,
  TAccountReserveStake extends string = string,
  TAccountDestinationLamports extends string = string,
  TAccountManagerFee extends string = string,
  TAccountPoolMint extends string = string,
  TAccountClock extends string = string,
  TAccountStakeHistory extends string = string,
  TAccountStakeProgram extends string = string,
  TAccountTokenProgram extends string = string,
> = {
  stakePool: Address<TAccountStakePool>;
  withdrawAuthority: Address<TAccountWithdrawAuthority>;
  userTransferAuthority: TransactionSigner<TAccountUserTransferAuthority>;
  burnFromPool: Address<TAccountBurnFromPool>;
  reserveStake: Address<TAccountReserveStake>;
  destinationLamports: Address<TAccountDestinationLamports>;
  managerFee: Address<TAccountManagerFee>;
  poolMint: Address<TAccountPoolMint>;
  clock: Address<TAccountClock>;
  stakeHistory: Address<TAccountStakeHistory>;
  stakeProgram: Address<TAccountStakeProgram>;
  tokenProgram: Address<TAccountTokenProgram>;
  args: WithdrawSolInstructionDataArgs['args'];
};

export function getWithdrawSolInstruction<
  TAccountStakePool extends string,
  TAccountWithdrawAuthority extends string,
  TAccountUserTransferAuthority extends string,
  TAccountBurnFromPool extends string,
  TAccountReserveStake extends string,
  TAccountDestinationLamports extends string,
  TAccountManagerFee extends string,
  TAccountPoolMint extends string,
  TAccountClock extends string,
  TAccountStakeHistory extends string,
  TAccountStakeProgram extends string,
  TAccountTokenProgram extends string,
  TProgramAddress extends Address = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
>(
  input: WithdrawSolInput<
    TAccountStakePool,
    TAccountWithdrawAuthority,
    TAccountUserTransferAuthority,
    TAccountBurnFromPool,
    TAccountReserveStake,
    TAccountDestinationLamports,
    TAccountManagerFee,
    TAccountPoolMint,
    TAccountClock,
    TAccountStakeHistory,
    TAccountStakeProgram,
    TAccountTokenProgram
  >,
  config?: { programAddress?: TProgramAddress }
): WithdrawSolInstruction<
  TProgramAddress,
  TAccountStakePool,
  TAccountWithdrawAuthority,
  TAccountUserTransferAuthority,
  TAccountBurnFromPool,
  TAccountReserveStake,
  TAccountDestinationLamports,
  TAccountManagerFee,
  TAccountPoolMint,
  TAccountClock,
  TAccountStakeHistory,
  TAccountStakeProgram,
  TAccountTokenProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? SPL_STAKE_POOL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    stakePool: { value: input.stakePool ?? null, isWritable: true },
    withdrawAuthority: {
      value: input.withdrawAuthority ?? null,
      isWritable: false,
    },
    userTransferAuthority: {
      value: input.userTransferAuthority ?? null,
      isWritable: false,
    },
    burnFromPool: { value: input.burnFromPool ?? null, isWritable: true },
    reserveStake: { value: input.reserveStake ?? null, isWritable: true },
    destinationLamports: {
      value: input.destinationLamports ?? null,
      isWritable: true,
    },
    managerFee: { value: input.managerFee ?? null, isWritable: true },
    poolMint: { value: input.poolMint ?? null, isWritable: true },
    clock: { value: input.clock ?? null, isWritable: false },
    stakeHistory: { value: input.stakeHistory ?? null, isWritable: false },
    stakeProgram: { value: input.stakeProgram ?? null, isWritable: false },
    tokenProgram: { value: input.tokenProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.stakePool),
      getAccountMeta(accounts.withdrawAuthority),
      getAccountMeta(accounts.userTransferAuthority),
      getAccountMeta(accounts.burnFromPool),
      getAccountMeta(accounts.reserveStake),
      getAccountMeta(accounts.destinationLamports),
      getAccountMeta(accounts.managerFee),
      getAccountMeta(accounts.poolMint),
      getAccountMeta(accounts.clock),
      getAccountMeta(accounts.stakeHistory),
      getAccountMeta(accounts.stakeProgram),
      getAccountMeta(accounts.tokenProgram),
    ],
    programAddress,
    data: getWithdrawSolInstructionDataEncoder().encode(
      args as WithdrawSolInstructionDataArgs
    ),
  } as WithdrawSolInstruction<
    TProgramAddress,
    TAccountStakePool,
    TAccountWithdrawAuthority,
    TAccountUserTransferAuthority,
    TAccountBurnFromPool,
    TAccountReserveStake,
    TAccountDestinationLamports,
    TAccountManagerFee,
    TAccountPoolMint,
    TAccountClock,
    TAccountStakeHistory,
    TAccountStakeProgram,
    TAccountTokenProgram
  >;

  return instruction;
}

export type ParsedWithdrawSolInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    stakePool: TAccountMetas[0];
    withdrawAuthority: TAccountMetas[1];
    userTransferAuthority: TAccountMetas[2];
    burnFromPool: TAccountMetas[3];
    reserveStake: TAccountMetas[4];
    destinationLamports: TAccountMetas[5];
    managerFee: TAccountMetas[6];
    poolMint: TAccountMetas[7];
    clock: TAccountMetas[8];
    stakeHistory: TAccountMetas[9];
    stakeProgram: TAccountMetas[10];
    tokenProgram: TAccountMetas[11];
  };
  data: WithdrawSolInstructionData;
};

export function parseWithdrawSolInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedWithdrawSolInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 12) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      stakePool: getNextAccount(),
      withdrawAuthority: getNextAccount(),
      userTransferAuthority: getNextAccount(),
      burnFromPool: getNextAccount(),
      reserveStake: getNextAccount(),
      destinationLamports: getNextAccount(),
      managerFee: getNextAccount(),
      poolMint: getNextAccount(),
      clock: getNextAccount(),
      stakeHistory: getNextAccount(),
      stakeProgram: getNextAccount(),
      tokenProgram: getNextAccount(),
    },
    data: getWithdrawSolInstructionDataDecoder().decode(instruction.data),
  };
}
