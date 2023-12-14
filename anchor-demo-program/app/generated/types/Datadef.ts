import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface NullJSON {
  kind: "Null"
}

export class Null {
  static readonly discriminator = 0
  static readonly kind = "Null"
  readonly discriminator = 0
  readonly kind = "Null"

  toJSON(): NullJSON {
    return {
      kind: "Null",
    }
  }

  toEncodable() {
    return {
      Null: {},
    }
  }
}

export type BooleanFields = {
  value: boolean
}
export type BooleanValue = {
  value: boolean
}

export interface BooleanJSON {
  kind: "Boolean"
  value: {
    value: boolean
  }
}

export class Boolean {
  static readonly discriminator = 1
  static readonly kind = "Boolean"
  readonly discriminator = 1
  readonly kind = "Boolean"
  readonly value: BooleanValue

  constructor(value: BooleanFields) {
    this.value = {
      value: value.value,
    }
  }

  toJSON(): BooleanJSON {
    return {
      kind: "Boolean",
      value: {
        value: this.value.value,
      },
    }
  }

  toEncodable() {
    return {
      Boolean: {
        value: this.value.value,
      },
    }
  }
}

export type ArrayFields = {
  value: Uint8Array
}
export type ArrayValue = {
  value: Uint8Array
}

export interface ArrayJSON {
  kind: "Array"
  value: {
    value: Array<number>
  }
}

export class Array {
  static readonly discriminator = 2
  static readonly kind = "Array"
  readonly discriminator = 2
  readonly kind = "Array"
  readonly value: ArrayValue

  constructor(value: ArrayFields) {
    this.value = {
      value: value.value,
    }
  }

  toJSON(): ArrayJSON {
    return {
      kind: "Array",
      value: {
        value: Array.from(this.value.value.values()),
      },
    }
  }

  toEncodable() {
    return {
      Array: {
        value: Buffer.from(
          this.value.value.buffer,
          this.value.value.byteOffset,
          this.value.value.length
        ),
      },
    }
  }
}

export type NumberFields = {
  value: BN
}
export type NumberValue = {
  value: BN
}

export interface NumberJSON {
  kind: "Number"
  value: {
    value: string
  }
}

export class Number {
  static readonly discriminator = 3
  static readonly kind = "Number"
  readonly discriminator = 3
  readonly kind = "Number"
  readonly value: NumberValue

  constructor(value: NumberFields) {
    this.value = {
      value: value.value,
    }
  }

  toJSON(): NumberJSON {
    return {
      kind: "Number",
      value: {
        value: this.value.value.toString(),
      },
    }
  }

  toEncodable() {
    return {
      Number: {
        value: this.value.value,
      },
    }
  }
}

export type StringFields = {
  value: string
}
export type StringValue = {
  value: string
}

export interface StringJSON {
  kind: "String"
  value: {
    value: string
  }
}

export class String {
  static readonly discriminator = 4
  static readonly kind = "String"
  readonly discriminator = 4
  readonly kind = "String"
  readonly value: StringValue

  constructor(value: StringFields) {
    this.value = {
      value: value.value,
    }
  }

  toJSON(): StringJSON {
    return {
      kind: "String",
      value: {
        value: this.value.value,
      },
    }
  }

  toEncodable() {
    return {
      String: {
        value: this.value.value,
      },
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.DatadefKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("Null" in obj) {
    return new Null()
  }
  if ("Boolean" in obj) {
    const val = obj["Boolean"]
    return new Boolean({
      value: val["value"],
    })
  }
  if ("Array" in obj) {
    const val = obj["Array"]
    return new Array({
      value: new Uint8Array(
        val["value"].buffer,
        val["value"].byteOffset,
        val["value"].length
      ),
    })
  }
  if ("Number" in obj) {
    const val = obj["Number"]
    return new Number({
      value: val["value"],
    })
  }
  if ("String" in obj) {
    const val = obj["String"]
    return new String({
      value: val["value"],
    })
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(obj: types.DatadefJSON): types.DatadefKind {
  switch (obj.kind) {
    case "Null": {
      return new Null()
    }
    case "Boolean": {
      return new Boolean({
        value: obj.value.value,
      })
    }
    case "Array": {
      return new Array({
        value: Uint8Array.from(obj.value.value),
      })
    }
    case "Number": {
      return new Number({
        value: new BN(obj.value.value),
      })
    }
    case "String": {
      return new String({
        value: obj.value.value,
      })
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "Null"),
    borsh.struct([borsh.bool("value")], "Boolean"),
    borsh.struct([borsh.vecU8("value")], "Array"),
    borsh.struct([borsh.u64("value")], "Number"),
    borsh.struct([borsh.str("value")], "String"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
