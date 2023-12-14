import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface BasicStructFields {
  counter: number
  name: string
}

export interface BasicStructJSON {
  counter: number
  name: string
}

export class BasicStruct {
  readonly counter: number
  readonly name: string

  constructor(fields: BasicStructFields) {
    this.counter = fields.counter
    this.name = fields.name
  }

  static layout(property?: string) {
    return borsh.struct([borsh.i32("counter"), borsh.str("name")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new BasicStruct({
      counter: obj.counter,
      name: obj.name,
    })
  }

  static toEncodable(fields: BasicStructFields) {
    return {
      counter: fields.counter,
      name: fields.name,
    }
  }

  toJSON(): BasicStructJSON {
    return {
      counter: this.counter,
      name: this.name,
    }
  }

  static fromJSON(obj: BasicStructJSON): BasicStruct {
    return new BasicStruct({
      counter: obj.counter,
      name: obj.name,
    })
  }

  toEncodable() {
    return BasicStruct.toEncodable(this)
  }
}
