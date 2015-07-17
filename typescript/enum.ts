enum Test {
  t1,
  t2,
  t3
}
/*{
  '0': 't1',
  '1': 't2',
  '2': 't3',
  't1': '0',
  't2': '1',
  't3': '2'
}*/

export = Test;

var a: Test = Test.t1; // var a = Test.t1;

// tsc enum.ts --module commonjs --target ES5
