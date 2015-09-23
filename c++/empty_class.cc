#include <cstdio>
#include <cassert>

class empty {
};

inline static bool operator==(const empty& e1, const empty& e2) {
  return &e1 == &e2;
}
inline static bool operator!=(const empty& e1, const empty& e2) {
  return !(e1 == e2);
}

int main(void) {
  empty e1;
  empty e2;

  printf("%s\n", e1 == e1 ? "true" : "false");
  printf("%s\n", e1 == e2 ? "true" : "false");

  assert(e1 != e2);
  assert(e1 == e1);
  assert(e2 == e2);

  return 0;
}
