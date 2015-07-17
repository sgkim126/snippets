#include <cstdio>
#include <string>
#include <vector>
#include <algorithm>

int main(void) {
  std::vector<std::string> items({"abcd", "ABC", "abc"});
  std::sort(items.begin(), items.end());

  for (const auto& item: items) {
    printf("%s\n", item.c_str());
  }
  return 0;
}
