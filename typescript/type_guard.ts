function type_guard_example(arg: string | string[]): string {
  if (typeof arg === 'string') {
    // return arg.join().substring(0, 5); // error TS2339: Property 'join' does not exist on type 'string'.
    return arg.substring(0, 3);
  } else {
    // return arg.substring(0, 3); // error TS2339: Property 'substring' does not exist on type 'string[]'.
  }

  if (arg instanceof Array) {
    // return arg.substring(0, 3); // error TS2339: Property 'substring' does not exist on type 'string[]'.
    return arg.join().substring(0, 5);
  } else {
    // return arg.join().substring(0, 5); // error TS2339: Property 'join' does not exist on type 'string'.
  }

  return null;
}
