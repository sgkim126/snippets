var a = [ 1,2,3 ];
var b = a.slice(0);

a.pop();

console.log(a); // [ 1, 2 ]
console.log(b); // [ 1, 2, 3 ]
