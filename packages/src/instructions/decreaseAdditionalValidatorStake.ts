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

export const DECREASE_ADDITIONAL_VALIDATOR_STAKE_DISCRIMINATOR = 20;

export function getDecreaseAdditionalValidatorStakeDiscriminatorBytes() {
  return getU8Encoder().encode(
    DECREASE_ADDITIONAL_VALIDATOR_STAKE_DISCRIMINATOR
  );
}

export type DecreaseAdditionalValidatorStakeInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountStakePool extends string | IAccountMeta<string> = string,
  TAccountStaker extends string | IAccountMeta<string> = string,
  TAccountWithdrawAuthority extends string | IAccountMeta<string> = string,
  TAccountValidatorList extends string | IAccountMeta<string> = string,
  TAccountReserveStake extends string | IAccountMeta<string> = string,
  TAccountValidatorStakeAccount extends string | IAccountMeta<string> = string,
  TAccountEphemeralStakeAccount extends string | IAccountMeta<string> = string,
  TAccountTransientStakeAccount extends string | IAccountMeta<string> = string,
  TAccountClock extends string | IAccountMeta<string> = string,
  TAccountStakeHistory extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends string | IAccountMeta<string> = string,
  TAccountStakeProgram extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountStakePool extends string
        ? ReadonlyAccount<TAccountStakePool>
        : TAccountStakePool,
      TAccountStaker extends string
        ? ReadonlySignerAccount<TAccountStaker> &
            IAccountSignerMeta<TAccountStaker>
        : TAccountStaker,
      TAccountWithdrawAuthority extends string
        ? ReadonlyAccount<TAccountWithdrawAuthority>
        : TAccountWithdrawAuthority,
      TAccountValidatorList extends string
        ? WritableAccount<TAccountValidatorList>
        : TAccountValidatorList,
      TAccountReserveStake extends string
        ? WritableAccount<TAccountReserveStake>
        : TAccountReserveStake,
      TAccountValidatorStakeAccount extends string
        ? WritableAccount<TAccountValidatorStakeAccount>
        : TAccountValidatorStakeAccount,
      TAccountEphemeralStakeAccount extends string
        ? WritableAccount<TAccountEphemeralStakeAccount>
        : TAccountEphemeralStakeAccount,
      TAccountTransientStakeAccount extends string
        ? WritableAccount<TAccountTransientStakeAccount>
        : TAccountTransientStakeAccount,
      TAccountClock extends string
        ? ReadonlyAccount<TAccountClock>
        : TAccountClock,
      TAccountStakeHistory extends string
        ? ReadonlyAccount<TAccountStakeHistory>
        : TAccountStakeHistory,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      TAccountStakeProgram extends string
        ? ReadonlyAccount<TAccountStakeProgram>
        : TAccountStakeProgram,
      ...TRemainingAccounts,
    ]
  >;

export type DecreaseAdditionalValidatorStakeInstructionData = {
  discriminator: number;
  lamports: bigint;
  transientStakeSeed: bigint;
  ephemeralStakeSeed: bigint;
};

export type DecreaseAdditionalValidatorStakeInstructionDataArgs = {
  lamports: number | bigint;
  transientStakeSeed: number | bigint;
  ephemeralStakeSeed: number | bigint;
};

export function getDecreaseAdditionalValidatorStakeInstructionDataEncoder(): Encoder<DecreaseAdditionalValidatorStakeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['lamports', getU64Encoder()],
      ['transientStakeSeed', getU64Encoder()],
      ['ephemeralStakeSeed', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: DECREASE_ADDITIONAL_VALIDATOR_STAKE_DISCRIMINATOR,
    })
  );
}

export function getDecreaseAdditionalValidatorStakeInstructionDataDecoder(): Decoder<DecreaseAdditionalValidatorStakeInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['lamports', getU64Decoder()],
    ['transientStakeSeed', getU64Decoder()],
    ['ephemeralStakeSeed', getU64Decoder()],
  ]);
}

export function getDecreaseAdditionalValidatorStakeInstructionDataCodec(): Codec<
  DecreaseAdditionalValidatorStakeInstructionDataArgs,
  DecreaseAdditionalValidatorStakeInstructionData
> {
  return combineCodec(
    getDecreaseAdditionalValidatorStakeInstructionDataEncoder(),
    getDecreaseAdditionalValidatorStakeInstructionDataDecoder()
  );
}

export type DecreaseAdditionalValidatorStakeInput<
  TAccountStakePool extends string = string,
  TAccountStaker extends string = string,
  TAccountWithdrawAuthority extends string = string,
  TAccountValidatorList extends string = string,
  TAccountReserveStake extends string = string,
  TAccountValidatorStakeAccount extends string = string,
  TAccountEphemeralStakeAccount extends string = string,
  TAccountTransientStakeAccount extends string = string,
  TAccountClock extends string = string,
  TAccountStakeHistory extends string = string,
  TAccountSystemProgram extends string = string,
  TAccountStakeProgram extends string = string,
> = {
  stakePool: Address<TAccountStakePool>;
  staker: TransactionSigner<TAccountStaker>;
  withdrawAuthority: Address<TAccountWithdrawAuthority>;
  validatorList: Address<TAccountValidatorList>;
  reserveStake: Address<TAccountReserveStake>;
  validatorStakeAccount: Address<TAccountValidatorStakeAccount>;
  ephemeralStakeAccount: Address<TAccountEphemeralStakeAccount>;
  transientStakeAccount: Address<TAccountTransientStakeAccount>;
  clock: Address<TAccountClock>;
  stakeHistory: Address<TAccountStakeHistory>;
  systemProgram: Address<TAccountSystemProgram>;
  stakeProgram: Address<TAccountStakeProgram>;
  lamports: DecreaseAdditionalValidatorStakeInstructionDataArgs['lamports'];
  transientStakeSeed: DecreaseAdditionalValidatorStakeInstructionDataArgs['transientStakeSeed'];
  ephemeralStakeSeed: DecreaseAdditionalValidatorStakeInstructionDataArgs['ephemeralStakeSeed'];
};

export function getDecreaseAdditionalValidatorStakeInstruction<
  TAccountStakePool extends string,
  TAccountStaker extends string,
  TAccountWithdrawAuthority extends string,
  TAccountValidatorList extends string,
  TAccountReserveStake extends string,
  TAccountValidatorStakeAccount extends string,
  TAccountEphemeralStakeAccount extends string,
  TAccountTransientStakeAccount extends string,
  TAccountClock extends string,
  TAccountStakeHistory extends string,
  TAccountSystemProgram extends string,
  TAccountStakeProgram extends string,
  TProgramAddress extends Address = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
>(
  input: DecreaseAdditionalValidatorStakeInput<
    TAccountStakePool,
    TAccountStaker,
    TAccountWithdrawAuthority,
    TAccountValidatorList,
    TAccountReserveStake,
    TAccountValidatorStakeAccount,
    TAccountEphemeralStakeAccount,
    TAccountTransientStakeAccount,
    TAccountClock,
    TAccountStakeHistory,
    TAccountSystemProgram,
    TAccountStakeProgram
  >,
  config?: { programAddress?: TProgramAddress }
): DecreaseAdditionalValidatorStakeInstruction<
  TProgramAddress,
  TAccountStakePool,
  TAccountStaker,
  TAccountWithdrawAuthority,
  TAccountValidatorList,
  TAccountReserveStake,
  TAccountValidatorStakeAccount,
  TAccountEphemeralStakeAccount,
  TAccountTransientStakeAccount,
  TAccountClock,
  TAccountStakeHistory,
  TAccountSystemProgram,
  TAccountStakeProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? SPL_STAKE_POOL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    stakePool: { value: input.stakePool ?? null, isWritable: false },
    staker: { value: input.staker ?? null, isWritable: false },
    withdrawAuthority: {
      value: input.withdrawAuthority ?? null,
      isWritable: false,
    },
    validatorList: { value: input.validatorList ?? null, isWritable: true },
    reserveStake: { value: input.reserveStake ?? null, isWritable: true },
    validatorStakeAccount: {
      value: input.validatorStakeAccount ?? null,
      isWritable: true,
    },
    ephemeralStakeAccount: {
      value: input.ephemeralStakeAccount ?? null,
      isWritable: true,
    },
    transientStakeAccount: {
      value: input.transientStakeAccount ?? null,
      isWritable: true,
    },
    clock: { value: input.clock ?? null, isWritable: false },
    stakeHistory: { value: input.stakeHistory ?? null, isWritable: false },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
    stakeProgram: { value: input.stakeProgram ?? null, isWritable: false },
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
      getAccountMeta(accounts.staker),
      getAccountMeta(accounts.withdrawAuthority),
      getAccountMeta(accounts.validatorList),
      getAccountMeta(accounts.reserveStake),
      getAccountMeta(accounts.validatorStakeAccount),
      getAccountMeta(accounts.ephemeralStakeAccount),
      getAccountMeta(accounts.transientStakeAccount),
      getAccountMeta(accounts.clock),
      getAccountMeta(accounts.stakeHistory),
      getAccountMeta(accounts.systemProgram),
      getAccountMeta(accounts.stakeProgram),
    ],
    programAddress,
    data: getDecreaseAdditionalValidatorStakeInstructionDataEncoder().encode(
      args as DecreaseAdditionalValidatorStakeInstructionDataArgs
    ),
  } as DecreaseAdditionalValidatorStakeInstruction<
    TProgramAddress,
    TAccountStakePool,
    TAccountStaker,
    TAccountWithdrawAuthority,
    TAccountValidatorList,
    TAccountReserveStake,
    TAccountValidatorStakeAccount,
    TAccountEphemeralStakeAccount,
    TAccountTransientStakeAccount,
    TAccountClock,
    TAccountStakeHistory,
    TAccountSystemProgram,
    TAccountStakeProgram
  >;

  return instruction;
}

export type ParsedDecreaseAdditionalValidatorStakeInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    stakePool: TAccountMetas[0];
    staker: TAccountMetas[1];
    withdrawAuthority: TAccountMetas[2];
    validatorList: TAccountMetas[3];
    reserveStake: TAccountMetas[4];
    validatorStakeAccount: TAccountMetas[5];
    ephemeralStakeAccount: TAccountMetas[6];
    transientStakeAccount: TAccountMetas[7];
    clock: TAccountMetas[8];
    stakeHistory: TAccountMetas[9];
    systemProgram: TAccountMetas[10];
    stakeProgram: TAccountMetas[11];
  };
  data: DecreaseAdditionalValidatorStakeInstructionData;
};

export function parseDecreaseAdditionalValidatorStakeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedDecreaseAdditionalValidatorStakeInstruction<TProgram, TAccountMetas> {
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
      staker: getNextAccount(),
      withdrawAuthority: getNextAccount(),
      validatorList: getNextAccount(),
      reserveStake: getNextAccount(),
      validatorStakeAccount: getNextAccount(),
      ephemeralStakeAccount: getNextAccount(),
      transientStakeAccount: getNextAccount(),
      clock: getNextAccount(),
      stakeHistory: getNextAccount(),
      systemProgram: getNextAccount(),
      stakeProgram: getNextAccount(),
    },
    data: getDecreaseAdditionalValidatorStakeInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
