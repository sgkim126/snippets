undefined = 3;  // 3
undefined;      // undefined.

var someStrangeFunction = function(x, undefined) {
    return undefined === x;
}

someStrangeFunction(1);          // false
someStrangeFunction(true);       // false
someStrangeFunction(0);          // false
someStrangeFunction(false);      // false
someStrangeFunction(null);       // false
someStrangeFunction(undefined);  // true

someStrangeFunction(1, 1);            // true
someStrangeFunction(true, true);      // true
someStrangeFunction(0, 0);            // true
someStrangeFunction(false, false);    // true
someStrangeFunction(null, null);      // true
someStrangeFunction(undefined, null); // false
