import * as Datadef from "./Datadef"

export { BasicStruct } from "./BasicStruct"
export type { BasicStructFields, BasicStructJSON } from "./BasicStruct"
export { Datadef }

export type DatadefKind =
  | Datadef.Null
  | Datadef.Boolean
  | Datadef.Array
  | Datadef.Number
  | Datadef.String
export type DatadefJSON =
  | Datadef.NullJSON
  | Datadef.BooleanJSON
  | Datadef.ArrayJSON
  | Datadef.NumberJSON
  | Datadef.StringJSON
