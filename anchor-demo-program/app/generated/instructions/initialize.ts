import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface InitializeAccounts {
  exampleAccount: PublicKey
  exampleAccountDescriptor: PublicKey
  signer: PublicKey
  systemProgram: PublicKey
}

/**
 * Initialize the program.
 * Generate and publish the schemas for:
 * - ExampleAccount
 * - CloudEvent
 * - NonAnchorExample
 */
export function initialize(
  accounts: InitializeAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.exampleAccount, isSigner: true, isWritable: true },
    {
      pubkey: accounts.exampleAccountDescriptor,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.signer, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([175, 175, 109, 31, 13, 152, 155, 237])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
