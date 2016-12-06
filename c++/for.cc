#include <cstdio>
#include <vector>

using std::vector;

void for_auto(vector<int>& v) {
  for (auto i: v) {
    i += 10;
  }
}

void for_auto_ref(vector<int>& v) {
  for (auto& i: v) {
    i += 10;
  }
}

void for_index(vector<int>& v) {
  const size_t SIZE = v.size();
  for (size_t i = 0; i < SIZE; i += 1) {
    v[i] += 10;
  }
}

void for_index_at(vector<int>& v) {
  const size_t SIZE = v.size();
  for (size_t i = 0; i < SIZE; i += 1) {
    v.at(i) += 10;
  }
}

void for_iterator(vector<int>& v) {
  const auto END = v.end();
  for (auto it = v.begin(); it != END; ++it) {
    *it += 10;
  }
}

int main(void) {
  vector<int> v = { 1, 2, 3, 4, 5 };
  for_auto(v);
  for (auto i: v) {
    printf("%d ", i);
  }
  printf("\n====\n");

  v = { 1, 2, 3, 4, 5 };
  for_auto_ref(v);
  for (auto i: v) {
    printf("%d ", i);
  }
  printf("\n====\n");

  v = { 1, 2, 3, 4, 5 };
  for_index(v);
  for (auto i: v) {
    printf("%d ", i);
  }
  printf("\n====\n");

  v = { 1, 2, 3, 4, 5 };
  for_index_at(v);
  for (auto i: v) {
    printf("%d ", i);
  }
  printf("\n====\n");

  v = { 1, 2, 3, 4, 5 };
  for_iterator(v);
  for (auto i: v) {
    printf("%d ", i);
  }
  printf("\n====\n");

  return 0;
}
