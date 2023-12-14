import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ExampleAccountFields {
  counter: number
  programId: string
  data: types.BasicStructFields
}

export interface ExampleAccountJSON {
  counter: number
  programId: string
  data: types.BasicStructJSON
}

/**
 * For Anchor accounts we simply need to derive the following traits:
 * - BorshSchema
 * - SchemaGeneratorAnchor
 */
export class ExampleAccount {
  readonly counter: number
  readonly programId: string
  readonly data: types.BasicStruct

  static readonly discriminator = Buffer.from([
    188, 13, 109, 204, 38, 99, 69, 246,
  ])

  static readonly layout = borsh.struct([
    borsh.i32("counter"),
    borsh.str("programId"),
    types.BasicStruct.layout("data"),
  ])

  constructor(fields: ExampleAccountFields) {
    this.counter = fields.counter
    this.programId = fields.programId
    this.data = new types.BasicStruct({ ...fields.data })
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<ExampleAccount | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<ExampleAccount | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): ExampleAccount {
    if (!data.slice(0, 8).equals(ExampleAccount.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = ExampleAccount.layout.decode(data.slice(8))

    return new ExampleAccount({
      counter: dec.counter,
      programId: dec.programId,
      data: types.BasicStruct.fromDecoded(dec.data),
    })
  }

  toJSON(): ExampleAccountJSON {
    return {
      counter: this.counter,
      programId: this.programId,
      data: this.data.toJSON(),
    }
  }

  static fromJSON(obj: ExampleAccountJSON): ExampleAccount {
    return new ExampleAccount({
      counter: obj.counter,
      programId: obj.programId,
      data: types.BasicStruct.fromJSON(obj.data),
    })
  }
}
