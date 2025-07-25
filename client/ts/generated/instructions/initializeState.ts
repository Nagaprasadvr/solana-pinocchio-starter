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
  type AccountMeta,
  type AccountSignerMeta,
  type Address,
  type FixedSizeCodec,
  type FixedSizeDecoder,
  type FixedSizeEncoder,
  type Instruction,
  type InstructionWithAccounts,
  type InstructionWithData,
  type ReadonlyAccount,
  type ReadonlyUint8Array,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/kit';
import { SOLANA_PINOCCHIO_STARTER_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';
import {
  getInitializeMyStateIxDataDecoder,
  getInitializeMyStateIxDataEncoder,
  type InitializeMyStateIxData,
  type InitializeMyStateIxDataArgs,
} from '../types';

export const INITIALIZE_STATE_DISCRIMINATOR = 0;

export function getInitializeStateDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_STATE_DISCRIMINATOR);
}

export type InitializeStateInstruction<
  TProgram extends string = typeof SOLANA_PINOCCHIO_STARTER_PROGRAM_ADDRESS,
  TAccountPayerAcc extends string | AccountMeta<string> = string,
  TAccountStateAcc extends string | AccountMeta<string> = string,
  TAccountSysvarRentAcc extends string | AccountMeta<string> = string,
  TAccountSystemProgramAcc extends string | AccountMeta<string> = string,
  TRemainingAccounts extends readonly AccountMeta<string>[] = [],
> = Instruction<TProgram> &
  InstructionWithData<ReadonlyUint8Array> &
  InstructionWithAccounts<
    [
      TAccountPayerAcc extends string
        ? WritableSignerAccount<TAccountPayerAcc> &
            AccountSignerMeta<TAccountPayerAcc>
        : TAccountPayerAcc,
      TAccountStateAcc extends string
        ? WritableAccount<TAccountStateAcc>
        : TAccountStateAcc,
      TAccountSysvarRentAcc extends string
        ? ReadonlyAccount<TAccountSysvarRentAcc>
        : TAccountSysvarRentAcc,
      TAccountSystemProgramAcc extends string
        ? ReadonlyAccount<TAccountSystemProgramAcc>
        : TAccountSystemProgramAcc,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeStateInstructionData = {
  discriminator: number;
  initializeMyStateIxData: InitializeMyStateIxData;
};

export type InitializeStateInstructionDataArgs = {
  initializeMyStateIxData: InitializeMyStateIxDataArgs;
};

export function getInitializeStateInstructionDataEncoder(): FixedSizeEncoder<InitializeStateInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['initializeMyStateIxData', getInitializeMyStateIxDataEncoder()],
    ]),
    (value) => ({ ...value, discriminator: INITIALIZE_STATE_DISCRIMINATOR })
  );
}

export function getInitializeStateInstructionDataDecoder(): FixedSizeDecoder<InitializeStateInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['initializeMyStateIxData', getInitializeMyStateIxDataDecoder()],
  ]);
}

export function getInitializeStateInstructionDataCodec(): FixedSizeCodec<
  InitializeStateInstructionDataArgs,
  InitializeStateInstructionData
> {
  return combineCodec(
    getInitializeStateInstructionDataEncoder(),
    getInitializeStateInstructionDataDecoder()
  );
}

export type InitializeStateInput<
  TAccountPayerAcc extends string = string,
  TAccountStateAcc extends string = string,
  TAccountSysvarRentAcc extends string = string,
  TAccountSystemProgramAcc extends string = string,
> = {
  /** Fee payer account */
  payerAcc: TransactionSigner<TAccountPayerAcc>;
  /** New State account */
  stateAcc: Address<TAccountStateAcc>;
  /** Sysvar rent account */
  sysvarRentAcc: Address<TAccountSysvarRentAcc>;
  /** System program account */
  systemProgramAcc: Address<TAccountSystemProgramAcc>;
  initializeMyStateIxData: InitializeStateInstructionDataArgs['initializeMyStateIxData'];
};

export function getInitializeStateInstruction<
  TAccountPayerAcc extends string,
  TAccountStateAcc extends string,
  TAccountSysvarRentAcc extends string,
  TAccountSystemProgramAcc extends string,
  TProgramAddress extends
    Address = typeof SOLANA_PINOCCHIO_STARTER_PROGRAM_ADDRESS,
>(
  input: InitializeStateInput<
    TAccountPayerAcc,
    TAccountStateAcc,
    TAccountSysvarRentAcc,
    TAccountSystemProgramAcc
  >,
  config?: { programAddress?: TProgramAddress }
): InitializeStateInstruction<
  TProgramAddress,
  TAccountPayerAcc,
  TAccountStateAcc,
  TAccountSysvarRentAcc,
  TAccountSystemProgramAcc
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? SOLANA_PINOCCHIO_STARTER_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    payerAcc: { value: input.payerAcc ?? null, isWritable: true },
    stateAcc: { value: input.stateAcc ?? null, isWritable: true },
    sysvarRentAcc: { value: input.sysvarRentAcc ?? null, isWritable: false },
    systemProgramAcc: {
      value: input.systemProgramAcc ?? null,
      isWritable: false,
    },
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
      getAccountMeta(accounts.payerAcc),
      getAccountMeta(accounts.stateAcc),
      getAccountMeta(accounts.sysvarRentAcc),
      getAccountMeta(accounts.systemProgramAcc),
    ],
    programAddress,
    data: getInitializeStateInstructionDataEncoder().encode(
      args as InitializeStateInstructionDataArgs
    ),
  } as InitializeStateInstruction<
    TProgramAddress,
    TAccountPayerAcc,
    TAccountStateAcc,
    TAccountSysvarRentAcc,
    TAccountSystemProgramAcc
  >;

  return instruction;
}

export type ParsedInitializeStateInstruction<
  TProgram extends string = typeof SOLANA_PINOCCHIO_STARTER_PROGRAM_ADDRESS,
  TAccountMetas extends readonly AccountMeta[] = readonly AccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Fee payer account */
    payerAcc: TAccountMetas[0];
    /** New State account */
    stateAcc: TAccountMetas[1];
    /** Sysvar rent account */
    sysvarRentAcc: TAccountMetas[2];
    /** System program account */
    systemProgramAcc: TAccountMetas[3];
  };
  data: InitializeStateInstructionData;
};

export function parseInitializeStateInstruction<
  TProgram extends string,
  TAccountMetas extends readonly AccountMeta[],
>(
  instruction: Instruction<TProgram> &
    InstructionWithAccounts<TAccountMetas> &
    InstructionWithData<ReadonlyUint8Array>
): ParsedInitializeStateInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 4) {
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
      payerAcc: getNextAccount(),
      stateAcc: getNextAccount(),
      sysvarRentAcc: getNextAccount(),
      systemProgramAcc: getNextAccount(),
    },
    data: getInitializeStateInstructionDataDecoder().decode(instruction.data),
  };
}
