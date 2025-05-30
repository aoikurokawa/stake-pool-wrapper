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
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type WritableAccount,
} from '@solana/kit';
import { SPL_STAKE_POOL_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const INCREASE_ADDITIONAL_VALIDATOR_STAKE_DISCRIMINATOR = 19;

export function getIncreaseAdditionalValidatorStakeDiscriminatorBytes() {
  return getU8Encoder().encode(
    INCREASE_ADDITIONAL_VALIDATOR_STAKE_DISCRIMINATOR
  );
}

export type IncreaseAdditionalValidatorStakeInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountStakePool extends string | IAccountMeta<string> = string,
  TAccountStaker extends string | IAccountMeta<string> = string,
  TAccountWithdrawAuthority extends string | IAccountMeta<string> = string,
  TAccountValidatorList extends string | IAccountMeta<string> = string,
  TAccountReserveStakeAccount extends string | IAccountMeta<string> = string,
  TAccountEphemeralStakeAccount extends string | IAccountMeta<string> = string,
  TAccountTransientStakeAccount extends string | IAccountMeta<string> = string,
  TAccountValidatorStakeAccount extends string | IAccountMeta<string> = string,
  TAccountValidatorVoteAccount extends string | IAccountMeta<string> = string,
  TAccountClock extends string | IAccountMeta<string> = string,
  TAccountStakeHistory extends string | IAccountMeta<string> = string,
  TAccountStakeConfig extends string | IAccountMeta<string> = string,
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
        ? WritableAccount<TAccountStaker>
        : TAccountStaker,
      TAccountWithdrawAuthority extends string
        ? ReadonlyAccount<TAccountWithdrawAuthority>
        : TAccountWithdrawAuthority,
      TAccountValidatorList extends string
        ? WritableAccount<TAccountValidatorList>
        : TAccountValidatorList,
      TAccountReserveStakeAccount extends string
        ? WritableAccount<TAccountReserveStakeAccount>
        : TAccountReserveStakeAccount,
      TAccountEphemeralStakeAccount extends string
        ? WritableAccount<TAccountEphemeralStakeAccount>
        : TAccountEphemeralStakeAccount,
      TAccountTransientStakeAccount extends string
        ? WritableAccount<TAccountTransientStakeAccount>
        : TAccountTransientStakeAccount,
      TAccountValidatorStakeAccount extends string
        ? ReadonlyAccount<TAccountValidatorStakeAccount>
        : TAccountValidatorStakeAccount,
      TAccountValidatorVoteAccount extends string
        ? ReadonlyAccount<TAccountValidatorVoteAccount>
        : TAccountValidatorVoteAccount,
      TAccountClock extends string
        ? ReadonlyAccount<TAccountClock>
        : TAccountClock,
      TAccountStakeHistory extends string
        ? ReadonlyAccount<TAccountStakeHistory>
        : TAccountStakeHistory,
      TAccountStakeConfig extends string
        ? ReadonlyAccount<TAccountStakeConfig>
        : TAccountStakeConfig,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      TAccountStakeProgram extends string
        ? ReadonlyAccount<TAccountStakeProgram>
        : TAccountStakeProgram,
      ...TRemainingAccounts,
    ]
  >;

export type IncreaseAdditionalValidatorStakeInstructionData = {
  discriminator: number;
  lamports: bigint;
  transientStakeSeed: bigint;
  ephemeralStakeSeed: bigint;
};

export type IncreaseAdditionalValidatorStakeInstructionDataArgs = {
  lamports: number | bigint;
  transientStakeSeed: number | bigint;
  ephemeralStakeSeed: number | bigint;
};

export function getIncreaseAdditionalValidatorStakeInstructionDataEncoder(): Encoder<IncreaseAdditionalValidatorStakeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['lamports', getU64Encoder()],
      ['transientStakeSeed', getU64Encoder()],
      ['ephemeralStakeSeed', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: INCREASE_ADDITIONAL_VALIDATOR_STAKE_DISCRIMINATOR,
    })
  );
}

export function getIncreaseAdditionalValidatorStakeInstructionDataDecoder(): Decoder<IncreaseAdditionalValidatorStakeInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['lamports', getU64Decoder()],
    ['transientStakeSeed', getU64Decoder()],
    ['ephemeralStakeSeed', getU64Decoder()],
  ]);
}

export function getIncreaseAdditionalValidatorStakeInstructionDataCodec(): Codec<
  IncreaseAdditionalValidatorStakeInstructionDataArgs,
  IncreaseAdditionalValidatorStakeInstructionData
> {
  return combineCodec(
    getIncreaseAdditionalValidatorStakeInstructionDataEncoder(),
    getIncreaseAdditionalValidatorStakeInstructionDataDecoder()
  );
}

export type IncreaseAdditionalValidatorStakeInput<
  TAccountStakePool extends string = string,
  TAccountStaker extends string = string,
  TAccountWithdrawAuthority extends string = string,
  TAccountValidatorList extends string = string,
  TAccountReserveStakeAccount extends string = string,
  TAccountEphemeralStakeAccount extends string = string,
  TAccountTransientStakeAccount extends string = string,
  TAccountValidatorStakeAccount extends string = string,
  TAccountValidatorVoteAccount extends string = string,
  TAccountClock extends string = string,
  TAccountStakeHistory extends string = string,
  TAccountStakeConfig extends string = string,
  TAccountSystemProgram extends string = string,
  TAccountStakeProgram extends string = string,
> = {
  stakePool: Address<TAccountStakePool>;
  staker: Address<TAccountStaker>;
  withdrawAuthority: Address<TAccountWithdrawAuthority>;
  validatorList: Address<TAccountValidatorList>;
  reserveStakeAccount: Address<TAccountReserveStakeAccount>;
  ephemeralStakeAccount: Address<TAccountEphemeralStakeAccount>;
  transientStakeAccount: Address<TAccountTransientStakeAccount>;
  validatorStakeAccount: Address<TAccountValidatorStakeAccount>;
  validatorVoteAccount: Address<TAccountValidatorVoteAccount>;
  clock: Address<TAccountClock>;
  stakeHistory: Address<TAccountStakeHistory>;
  stakeConfig: Address<TAccountStakeConfig>;
  systemProgram: Address<TAccountSystemProgram>;
  stakeProgram: Address<TAccountStakeProgram>;
  lamports: IncreaseAdditionalValidatorStakeInstructionDataArgs['lamports'];
  transientStakeSeed: IncreaseAdditionalValidatorStakeInstructionDataArgs['transientStakeSeed'];
  ephemeralStakeSeed: IncreaseAdditionalValidatorStakeInstructionDataArgs['ephemeralStakeSeed'];
};

export function getIncreaseAdditionalValidatorStakeInstruction<
  TAccountStakePool extends string,
  TAccountStaker extends string,
  TAccountWithdrawAuthority extends string,
  TAccountValidatorList extends string,
  TAccountReserveStakeAccount extends string,
  TAccountEphemeralStakeAccount extends string,
  TAccountTransientStakeAccount extends string,
  TAccountValidatorStakeAccount extends string,
  TAccountValidatorVoteAccount extends string,
  TAccountClock extends string,
  TAccountStakeHistory extends string,
  TAccountStakeConfig extends string,
  TAccountSystemProgram extends string,
  TAccountStakeProgram extends string,
  TProgramAddress extends Address = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
>(
  input: IncreaseAdditionalValidatorStakeInput<
    TAccountStakePool,
    TAccountStaker,
    TAccountWithdrawAuthority,
    TAccountValidatorList,
    TAccountReserveStakeAccount,
    TAccountEphemeralStakeAccount,
    TAccountTransientStakeAccount,
    TAccountValidatorStakeAccount,
    TAccountValidatorVoteAccount,
    TAccountClock,
    TAccountStakeHistory,
    TAccountStakeConfig,
    TAccountSystemProgram,
    TAccountStakeProgram
  >,
  config?: { programAddress?: TProgramAddress }
): IncreaseAdditionalValidatorStakeInstruction<
  TProgramAddress,
  TAccountStakePool,
  TAccountStaker,
  TAccountWithdrawAuthority,
  TAccountValidatorList,
  TAccountReserveStakeAccount,
  TAccountEphemeralStakeAccount,
  TAccountTransientStakeAccount,
  TAccountValidatorStakeAccount,
  TAccountValidatorVoteAccount,
  TAccountClock,
  TAccountStakeHistory,
  TAccountStakeConfig,
  TAccountSystemProgram,
  TAccountStakeProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? SPL_STAKE_POOL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    stakePool: { value: input.stakePool ?? null, isWritable: false },
    staker: { value: input.staker ?? null, isWritable: true },
    withdrawAuthority: {
      value: input.withdrawAuthority ?? null,
      isWritable: false,
    },
    validatorList: { value: input.validatorList ?? null, isWritable: true },
    reserveStakeAccount: {
      value: input.reserveStakeAccount ?? null,
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
    validatorStakeAccount: {
      value: input.validatorStakeAccount ?? null,
      isWritable: false,
    },
    validatorVoteAccount: {
      value: input.validatorVoteAccount ?? null,
      isWritable: false,
    },
    clock: { value: input.clock ?? null, isWritable: false },
    stakeHistory: { value: input.stakeHistory ?? null, isWritable: false },
    stakeConfig: { value: input.stakeConfig ?? null, isWritable: false },
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
      getAccountMeta(accounts.reserveStakeAccount),
      getAccountMeta(accounts.ephemeralStakeAccount),
      getAccountMeta(accounts.transientStakeAccount),
      getAccountMeta(accounts.validatorStakeAccount),
      getAccountMeta(accounts.validatorVoteAccount),
      getAccountMeta(accounts.clock),
      getAccountMeta(accounts.stakeHistory),
      getAccountMeta(accounts.stakeConfig),
      getAccountMeta(accounts.systemProgram),
      getAccountMeta(accounts.stakeProgram),
    ],
    programAddress,
    data: getIncreaseAdditionalValidatorStakeInstructionDataEncoder().encode(
      args as IncreaseAdditionalValidatorStakeInstructionDataArgs
    ),
  } as IncreaseAdditionalValidatorStakeInstruction<
    TProgramAddress,
    TAccountStakePool,
    TAccountStaker,
    TAccountWithdrawAuthority,
    TAccountValidatorList,
    TAccountReserveStakeAccount,
    TAccountEphemeralStakeAccount,
    TAccountTransientStakeAccount,
    TAccountValidatorStakeAccount,
    TAccountValidatorVoteAccount,
    TAccountClock,
    TAccountStakeHistory,
    TAccountStakeConfig,
    TAccountSystemProgram,
    TAccountStakeProgram
  >;

  return instruction;
}

export type ParsedIncreaseAdditionalValidatorStakeInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    stakePool: TAccountMetas[0];
    staker: TAccountMetas[1];
    withdrawAuthority: TAccountMetas[2];
    validatorList: TAccountMetas[3];
    reserveStakeAccount: TAccountMetas[4];
    ephemeralStakeAccount: TAccountMetas[5];
    transientStakeAccount: TAccountMetas[6];
    validatorStakeAccount: TAccountMetas[7];
    validatorVoteAccount: TAccountMetas[8];
    clock: TAccountMetas[9];
    stakeHistory: TAccountMetas[10];
    stakeConfig: TAccountMetas[11];
    systemProgram: TAccountMetas[12];
    stakeProgram: TAccountMetas[13];
  };
  data: IncreaseAdditionalValidatorStakeInstructionData;
};

export function parseIncreaseAdditionalValidatorStakeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedIncreaseAdditionalValidatorStakeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 14) {
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
      reserveStakeAccount: getNextAccount(),
      ephemeralStakeAccount: getNextAccount(),
      transientStakeAccount: getNextAccount(),
      validatorStakeAccount: getNextAccount(),
      validatorVoteAccount: getNextAccount(),
      clock: getNextAccount(),
      stakeHistory: getNextAccount(),
      stakeConfig: getNextAccount(),
      systemProgram: getNextAccount(),
      stakeProgram: getNextAccount(),
    },
    data: getIncreaseAdditionalValidatorStakeInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
