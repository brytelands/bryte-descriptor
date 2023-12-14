import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface UpdateAccountArgs {
  newName: string
}

export interface UpdateAccountAccounts {
  exampleAccount: PublicKey
  signer: PublicKey
}

export const layout = borsh.struct([borsh.str("newName")])

/**
 * Update the account.
 * Log (emit! or publish!) the following events:
 * - CloudEvent
 * - NonAnchorExample
 */
export function updateAccount(
  args: UpdateAccountArgs,
  accounts: UpdateAccountAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.exampleAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.signer, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([231, 31, 72, 97, 68, 133, 133, 152])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      newName: args.newName,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
