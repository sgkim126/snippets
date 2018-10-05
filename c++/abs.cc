#include <cmath>
#include <iostream>
#include <limits>

int main(void) {
  int i = std::numeric_limits<int>::min();
  std::cout << i << std::endl;
  std::cout << std::abs(i) << std::endl;
  double d = i;
  std::cout << d << std::endl;
  std::cout << std::abs(d) << std::endl;
}
