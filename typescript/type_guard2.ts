function type_guard_example(arg: string | number | string[]): number {
  if (typeof arg === 'string' || arg instanceof Array) {
    return arg.length; // arg: string | string[]
  }

  return arg; // error TS2322: Type 'string | number | string[]' is not assignable to type 'number'.
}
