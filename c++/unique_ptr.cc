#include <cstdio>
#include <memory>

class A {
 private:
  int id;

 public:
  A(int id): id(id) {
    printf("constructed %d\n", this->id);
  }
  ~A() {
    printf("destructed %d\n", this->id);
  }
};

int main(void) {
  std::unique_ptr<A> a = std::unique_ptr<A>(new A(0));

  std::unique_ptr<A> b = nullptr;
  printf("%p %s\n", a.get(), a ? "true" : "false");
  printf("%p %s\n", b.get(), b ? "true" : "false");

  b.reset(new A(1));

  a = std::unique_ptr<A>(new A(2)); 
  b.reset(new A(3));
  return 0;
}
