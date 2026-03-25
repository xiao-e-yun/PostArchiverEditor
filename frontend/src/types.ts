export * from "../../bindings/WithRelations"
export * from "../../bindings/PostResponse"
export * from "../../bindings/PostShortResponse"

export type Totalled<T> = {
  items: T[];
  total: number;
}
