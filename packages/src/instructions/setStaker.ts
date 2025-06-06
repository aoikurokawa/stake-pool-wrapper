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

export const SET_STAKER_DISCRIMINATOR = 13;

export function getSetStakerDiscriminatorBytes() {
  return getU8Encoder().encode(SET_STAKER_DISCRIMINATOR);
}

export type SetStakerInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountStakePool extends string | IAccountMeta<string> = string,
  TAccountManager extends string | IAccountMeta<string> = string,
  TAccountNewStaker extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountStakePool extends string
        ? WritableAccount<TAccountStakePool>
        : TAccountStakePool,
      TAccountManager extends string
        ? ReadonlySignerAccount<TAccountManager> &
            IAccountSignerMeta<TAccountManager>
        : TAccountManager,
      TAccountNewStaker extends string
        ? ReadonlyAccount<TAccountNewStaker>
        : TAccountNewStaker,
      ...TRemainingAccounts,
    ]
  >;

export type SetStakerInstructionData = { discriminator: number };

export type SetStakerInstructionDataArgs = {};

export function getSetStakerInstructionDataEncoder(): Encoder<SetStakerInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: SET_STAKER_DISCRIMINATOR })
  );
}

export function getSetStakerInstructionDataDecoder(): Decoder<SetStakerInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getSetStakerInstructionDataCodec(): Codec<
  SetStakerInstructionDataArgs,
  SetStakerInstructionData
> {
  return combineCodec(
    getSetStakerInstructionDataEncoder(),
    getSetStakerInstructionDataDecoder()
  );
}

export type SetStakerInput<
  TAccountStakePool extends string = string,
  TAccountManager extends string = string,
  TAccountNewStaker extends string = string,
> = {
  stakePool: Address<TAccountStakePool>;
  manager: TransactionSigner<TAccountManager>;
  newStaker: Address<TAccountNewStaker>;
};

export function getSetStakerInstruction<
  TAccountStakePool extends string,
  TAccountManager extends string,
  TAccountNewStaker extends string,
  TProgramAddress extends Address = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
>(
  input: SetStakerInput<TAccountStakePool, TAccountManager, TAccountNewStaker>,
  config?: { programAddress?: TProgramAddress }
): SetStakerInstruction<
  TProgramAddress,
  TAccountStakePool,
  TAccountManager,
  TAccountNewStaker
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? SPL_STAKE_POOL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    stakePool: { value: input.stakePool ?? null, isWritable: true },
    manager: { value: input.manager ?? null, isWritable: false },
    newStaker: { value: input.newStaker ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.stakePool),
      getAccountMeta(accounts.manager),
      getAccountMeta(accounts.newStaker),
    ],
    programAddress,
    data: getSetStakerInstructionDataEncoder().encode({}),
  } as SetStakerInstruction<
    TProgramAddress,
    TAccountStakePool,
    TAccountManager,
    TAccountNewStaker
  >;

  return instruction;
}

export type ParsedSetStakerInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    stakePool: TAccountMetas[0];
    manager: TAccountMetas[1];
    newStaker: TAccountMetas[2];
  };
  data: SetStakerInstructionData;
};

export function parseSetStakerInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedSetStakerInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
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
      manager: getNextAccount(),
      newStaker: getNextAccount(),
    },
    data: getSetStakerInstructionDataDecoder().decode(instruction.data),
  };
}
