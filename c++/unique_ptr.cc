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
  b = std::unique_ptr<A>(new A(1));

  a = std::unique_ptr<A>(new A(2)); 
  return 0;
}
