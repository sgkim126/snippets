#include <cstdio>

class A {
public:
  A() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
  virtual ~A() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
};
class B {
public:
  B() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
  ~B() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
};
class C {
public:
  C() {
    printf("%s\n", __PRETTY_FUNCTION__);
    throw 0;
  }
  ~C() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
};
class D {
public:
  D() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
  ~D() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
};
class E: public A {
public:
  E() : A(), b(), c(), d() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
  ~E() {
    printf("%s\n", __PRETTY_FUNCTION__);
  }
private:
  B b;
  C c;
  D d;
};

int main(void) {
  try {
    E e;
  } catch (...) {
  }
  return 0;
}
