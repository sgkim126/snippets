class Base {
  static static_method1() {
    return console.log('base static method 1');
  }

  static static_method2() {
    return console.log('base static method 2');
  }

  prototype_method1() {
    return console.log('base prototype method 1');
  }

  prototype_method2() {
    return console.log('base prototype method 2');
  }
}

class Derived extends Base {
  static static_method1() {
    return console.log('derived static method 1');
  }

  prototype_method1() {
    return console.log('derived prototype method 1');
  }
}

Base.static_method1(); // base static method 1
Base.static_method2(); // base static method 2
let b = new Base();
b.prototype_method1(); // base prototype method 1
b.prototype_method2(); // base prototype method 2

Derived.static_method1(); // derived static method 1
Derived.static_method2(); // base static method 2
let d = new Derived();
d.prototype_method1(); // derived prototype method 1
d.prototype_method2(); // base prototype method 2

let BaseClassExpression = class {
  static static_method3() {
    return console.log('base static method 3');
  }

  static static_method4() {
    return console.log('base static method 4');
  }

  prototype_method3() {
    return console.log('base prototype method 3');
  }

  prototype_method4() {
    return console.log('base prototype method 4');
  }
};

let DerivedClassExpression = class extends BaseClassExpression {
  static static_method3() {
    return console.log('derived static method 3');
  }

  prototype_method3() {
    return console.log('derived prototype method 3');
  }
};
