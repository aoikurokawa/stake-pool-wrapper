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
import {
  getFundingTypeDecoder,
  getFundingTypeEncoder,
  type FundingType,
  type FundingTypeArgs,
} from '../types';

export const SET_FUNDING_AUTHORITY_DISCRIMINATOR = 15;

export function getSetFundingAuthorityDiscriminatorBytes() {
  return getU8Encoder().encode(SET_FUNDING_AUTHORITY_DISCRIMINATOR);
}

export type SetFundingAuthorityInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountStakePool extends string | IAccountMeta<string> = string,
  TAccountManager extends string | IAccountMeta<string> = string,
  TAccountNewAuthority extends string | IAccountMeta<string> = string,
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
      TAccountNewAuthority extends string
        ? ReadonlyAccount<TAccountNewAuthority>
        : TAccountNewAuthority,
      ...TRemainingAccounts,
    ]
  >;

export type SetFundingAuthorityInstructionData = {
  discriminator: number;
  args: FundingType;
};

export type SetFundingAuthorityInstructionDataArgs = { args: FundingTypeArgs };

export function getSetFundingAuthorityInstructionDataEncoder(): Encoder<SetFundingAuthorityInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['args', getFundingTypeEncoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: SET_FUNDING_AUTHORITY_DISCRIMINATOR,
    })
  );
}

export function getSetFundingAuthorityInstructionDataDecoder(): Decoder<SetFundingAuthorityInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['args', getFundingTypeDecoder()],
  ]);
}

export function getSetFundingAuthorityInstructionDataCodec(): Codec<
  SetFundingAuthorityInstructionDataArgs,
  SetFundingAuthorityInstructionData
> {
  return combineCodec(
    getSetFundingAuthorityInstructionDataEncoder(),
    getSetFundingAuthorityInstructionDataDecoder()
  );
}

export type SetFundingAuthorityInput<
  TAccountStakePool extends string = string,
  TAccountManager extends string = string,
  TAccountNewAuthority extends string = string,
> = {
  stakePool: Address<TAccountStakePool>;
  manager: TransactionSigner<TAccountManager>;
  newAuthority: Address<TAccountNewAuthority>;
  args: SetFundingAuthorityInstructionDataArgs['args'];
};

export function getSetFundingAuthorityInstruction<
  TAccountStakePool extends string,
  TAccountManager extends string,
  TAccountNewAuthority extends string,
  TProgramAddress extends Address = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
>(
  input: SetFundingAuthorityInput<
    TAccountStakePool,
    TAccountManager,
    TAccountNewAuthority
  >,
  config?: { programAddress?: TProgramAddress }
): SetFundingAuthorityInstruction<
  TProgramAddress,
  TAccountStakePool,
  TAccountManager,
  TAccountNewAuthority
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? SPL_STAKE_POOL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    stakePool: { value: input.stakePool ?? null, isWritable: true },
    manager: { value: input.manager ?? null, isWritable: false },
    newAuthority: { value: input.newAuthority ?? null, isWritable: false },
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
      getAccountMeta(accounts.manager),
      getAccountMeta(accounts.newAuthority),
    ],
    programAddress,
    data: getSetFundingAuthorityInstructionDataEncoder().encode(
      args as SetFundingAuthorityInstructionDataArgs
    ),
  } as SetFundingAuthorityInstruction<
    TProgramAddress,
    TAccountStakePool,
    TAccountManager,
    TAccountNewAuthority
  >;

  return instruction;
}

export type ParsedSetFundingAuthorityInstruction<
  TProgram extends string = typeof SPL_STAKE_POOL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    stakePool: TAccountMetas[0];
    manager: TAccountMetas[1];
    newAuthority: TAccountMetas[2];
  };
  data: SetFundingAuthorityInstructionData;
};

export function parseSetFundingAuthorityInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedSetFundingAuthorityInstruction<TProgram, TAccountMetas> {
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
      newAuthority: getNextAccount(),
    },
    data: getSetFundingAuthorityInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
