#include <cstdint>
#include <thread>

static uint32_t g_count =0;
void increase(uint32_t tid, uint32_t n) {
  for (uint32_t i = 0; i < n; i += 1) {
    g_count += 1;
  }
}

int main(void) {
  auto&& t = std::thread(increase, 0, 100);

  t.join();

  printf("%u\n", g_count);

  return 0;
}
