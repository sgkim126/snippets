#include <atomic>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <mutex>
#include <string>
#include <thread>
#include <vector>

#include <getopt.h>
#include <semaphore.h>


#if defined(__GNUC__)
#define INLINE inline __attribute__((__always_inline__))
#elif defined(_MSC_VER)
#define INLINE __forceinline
#else
#define INLINE inline
#endif

static INLINE void memory_fence(void)
{
#if defined(__GNUC__)
    __sync_synchronize();
#elif defined(__llvm__)
    __sync_synchronize();
#elif defined(_MSC_VER)
    _mm_mfence();
#else
#error This compiler is not supported.
#endif
}


static void print_usage(FILE* out, const char* const name) {
  fprintf(out, "Usage: %s [options]\n", name);
  fprintf(out, "option:\n");
  fprintf(out, "\t-t {number}\tthe number of threads (default is 8)\n");
  fprintf(out, "\t-n {number}\tthe number to add (default is 1,000,000\n");
  fprintf(out, "\t-l [pthread|filter|cpp11|buggy|atomic]\tthe type of lock (default is pthread)\n");
  fprintf(out, "\t-h\tprint this message\n");
}

enum class lock_type {
  pthread, filter, cpp11, buggy, atomic
};

struct config {
  uint32_t number_of_threads;
  uint32_t number_to_add;
  ::lock_type lock_type;

  config()
      : number_of_threads(8),
      number_to_add(1000000),
      lock_type(::lock_type::pthread) {
  }

  config(uint32_t number_of_threads, uint32_t number_to_add, ::lock_type lock_type)
      : number_of_threads(number_of_threads),
      number_to_add(number_to_add),
      lock_type(lock_type) {
  }
};

static config parse_argument(int argc, char * const argv[]) {
  config config;

  int opt;
  while ((opt = getopt(argc, argv, "ht:n:l:")) != -1) {
  switch (opt) {
  case 't': {
    int opt_number_of_threads = atoi(optarg);
    if (opt_number_of_threads <= 0) {
      print_usage(stderr, argv[0]);
      exit(-1);
    }
    config.number_of_threads = static_cast<uint32_t>(opt_number_of_threads);
    break;
  }
  case 'n': {
    int opt_number_to_add = atoi(optarg);
    if (opt_number_to_add <= 0) {
      print_usage(stderr, argv[0]);
      exit(-1);
    }
    config.number_to_add = static_cast<uint32_t>(opt_number_to_add);
    break;
  }
  case 'l': {
    std::string opt_lock_type(optarg);
    if (opt_lock_type == "pthread") {
      config.lock_type = ::lock_type::pthread;
    } else if (opt_lock_type == "filter") {
      config.lock_type = ::lock_type::filter;
    } else if (opt_lock_type == "cpp11") {
      config.lock_type = ::lock_type::cpp11;
    } else if (opt_lock_type == "buggy") {
      config.lock_type = ::lock_type::buggy;
    } else if (opt_lock_type == "atomic") {
      config.lock_type = ::lock_type::atomic;
    } else {
      print_usage(stderr, argv[0]);
      exit(-1);
    }
    break;
  }
  case 'h': {
    print_usage(stdout, argv[0]);
    exit(0);
    break;
  }
  default:
    break;
  }
  }

  return config;
}

static uint32_t g_count =0;


static uint32_t* filter_level;
static uint32_t* filter_victim;
static uint32_t filter_n;

static void filter_lock(uint32_t tid) {
  for (uint32_t L = 1; L < filter_n; L += 1) {
    filter_level[tid] = L;
    filter_victim[L] = tid;

    const auto&& filter_condition = [&L, &tid]() -> bool {
      for (uint32_t k = 0; k < filter_n; k += 1) {
        if (k == tid) {
          continue;
        }
        memory_fence();
        if (filter_level[k] >= L) {
          return true;
        }
      }
      return false;
    };
    while (true) {
      if (!filter_condition()) {
        break;
      }
      memory_fence();
      if (filter_victim[L] != tid) {
        break;
      }
    }
  }
}

static void filter_unlock(uint32_t tid) {
  filter_level[tid] = 0;
}

void increase_with_filter(uint32_t tid, uint32_t n) {
  for (uint32_t i = 0; i < n; i += 1) {
    filter_lock(tid);
    g_count += 1;
    filter_unlock(tid);
  }
}


static std::mutex mutex;
void increase_with_cpp11(uint32_t n) {
  for (uint32_t i = 0; i < n; i += 1) {
    std::lock_guard<std::mutex> lock(mutex);
    g_count += 1;
  }
}

static sem_t semaphore;
void increase_with_semaphore(uint32_t n) {
  for (uint32_t i = 0; i < n; i += 1) {
    sem_wait(&semaphore);
    g_count += 1;
    sem_post(&semaphore);
  }
}

void increase_with_buggy_implementation(uint32_t n) {
  for (uint32_t i = 0; i < n; i += 1) {
    g_count += 1;
  }
}


static std::atomic<uint32_t> g_atomic_count;
void increase_with_atomic(uint32_t n) {
  for (uint32_t i = 0; i < n; i += 1) {
    g_atomic_count += 1;
  }
}

int main(int argc, char * const argv[]) {
  const auto&& config = parse_argument(argc, argv);

  std::vector<std::thread> threads;
  switch (config.lock_type) {
  case ::lock_type::filter: {
    filter_n = config.number_of_threads;
    filter_level = new uint32_t[filter_n];
    filter_victim = new uint32_t[filter_n];
    for (size_t i = 0; i < config.number_of_threads; i += 1) {
      threads.emplace_back(increase_with_filter, i, config.number_to_add / config.number_of_threads);
    }
    break;
  }
  case ::lock_type::cpp11: {
    std::mutex mutex;
    for (size_t i = 0; i < config.number_of_threads; i += 1) {
      threads.emplace_back(increase_with_cpp11, config.number_to_add / config.number_of_threads);
    }
    break;
  }
  case ::lock_type::pthread: {
    sem_init(&semaphore, 0, 1);
    for (size_t i = 0; i < config.number_of_threads; i += 1) {
      threads.emplace_back(increase_with_semaphore, config.number_to_add / config.number_of_threads);
    }
    break;
  }
  case ::lock_type::buggy: {
    for (size_t i = 0; i < config.number_of_threads; i += 1) {
      threads.emplace_back(increase_with_buggy_implementation, config.number_to_add / config.number_of_threads);
    }
    break;
  }
  case ::lock_type::atomic: {
    for (size_t i = 0; i < config.number_of_threads; i += 1) {
      threads.emplace_back(increase_with_atomic, config.number_to_add / config.number_of_threads);
    }
    break;
  }
  default:
    assert(false);
    break;
  }

  for (auto& thread : threads) {
    thread.join();
  }

  if (config.lock_type == ::lock_type::atomic) {
    printf("Count: %u (%s)\n", static_cast<uint32_t>(g_atomic_count), g_atomic_count.is_lock_free() ? "true" : "false");
  } else {
    printf("Count: %u\n", g_count);
  }

  return 0;
}
