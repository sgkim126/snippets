const assert = require('node:assert/strict');

const p1 = Promise.resolve(1);
const p2 = Promise.resolve(1);
const p3 = Promise.resolve(p1);

assert.notEqual(p1, p2);
assert.equal(p1, p3);

const p4 = (() => p1)();
assert.equal(p1, p4);

const p5 = (() => Promise.resolve(p1))();
assert.equal(p1, p7);

// even though the return value of an async function behaves as if it's
// wrapped in a `Promise.resolve`, they are not equivalent. An async function
// will return a different reference, whereas Promise.resolve returns the same
// reference if the given value is a promise.
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function#description
const p6 = (async () => p1)();
const p7 = (async () => await p1)();
assert.notEqual(p1, p5);
assert.notEqual(p1, p6);
