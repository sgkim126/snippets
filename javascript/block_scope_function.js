var assert = require('assert');

function foo() {
  return 1;
}

{
  function foo() {
    return 2;
  }
  assert.equal(foo(), 2);
}

assert.equal(foo(), 1);
