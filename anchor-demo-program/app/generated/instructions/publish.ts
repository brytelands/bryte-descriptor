import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface PublishAccounts {
  exampleAccount: PublicKey
}

export function publish(
  accounts: PublishAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.exampleAccount, isSigner: false, isWritable: true },
  ]
  const identifier = Buffer.from([129, 177, 182, 160, 184, 224, 219, 5])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
