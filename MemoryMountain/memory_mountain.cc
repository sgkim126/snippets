#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <memory>
#include <utility>


#define KB 1024
#define MB (KB * KB)

#define THOUSAND 1000
#define MILLION (THOUSAND * THOUSAND)
#define BILLION (MILLION * THOUSAND)

void flush_cache() {
  const uint32_t large_memory_size = 10 * MB;
  static uint8_t *large_memory = new uint8_t[large_memory_size];

  for (int i = 0; i < large_memory_size; ++i) {
    large_memory[i] += 1;
  }
}

double metrics(const uint32_t metrix_size, const uint32_t stride) {
  std::unique_ptr<uint8_t[]> metrix1(new uint8_t[metrix_size]);
  std::unique_ptr<uint8_t[]> metrix2(new uint8_t[metrix_size]);

  const uint32_t count = metrix_size / stride;

  flush_cache();

  struct timespec begin, end;

  clock_gettime(CLOCK_REALTIME, &begin);

  for (auto i = 0; i < count; i++) {
    for (auto j = 0; j < stride; j++) {
      metrix1[i * stride + j] += 1;
    }
  }

  clock_gettime(CLOCK_REALTIME, &end);

  return static_cast<double>(count * stride)
      / ((end.tv_sec - begin.tv_sec) * BILLION
         + (end.tv_nsec - begin.tv_nsec)) * MILLION;
}

int main(int argc, char** argv) {
  static const uint32_t base_stride = 4 * 8;
  static const uint32_t max_stride = 64 * 8;

  static const uint32_t base_memory_size = 16 * KB;
  static const uint32_t max_memory_size = 32 * MB;

  printf("z = [");
  for (auto memory_size = base_memory_size; memory_size <= max_memory_size; memory_size *= 2) {
    printf("[");
    for (auto stride = base_stride; stride <= max_stride; stride += base_stride) {
      auto result = metrics(memory_size, stride);
      printf("%f,", result);
    }
    printf("],");
    fflush(stdout);
  }
  printf("]\n");
  return 0;
}
