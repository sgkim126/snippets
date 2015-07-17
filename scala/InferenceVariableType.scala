class A
class B extends A
class C extends A

var a: A = new B()

a = new C()
// It works. Because a is A explicitly.

var b = new B()

b = new C()
// error: type mismatch;
//  found   : this.C
//  required: this.B
// Because variable a is inferenced as B on line 5.
