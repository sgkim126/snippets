#include <iostream>

template <int N> int plus_n (int n) {
  return n + N;
}

int main(void) {
  std::cout << plus_n<1>(5) << std::endl;
  std::cout << plus_n<2>(5) << std::endl;
  std::cout << plus_n<3>(5) << std::endl;

  return 0;
}

