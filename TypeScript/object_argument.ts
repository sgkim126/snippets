function a(arg: { [key: string]: number }): { [key: string]: number } {
  return arg;
}

a({one: 1, two: 2, three: 3});
// a({TRUE: true, FALSE: false}); -> Argument of type '{ [x: string]: boolean; TRUE: boolean; FALSE: boolean; }' is not assignable to parameter of type '{ [key: string]: number; }'.
