#include <cassert>
#include <cstdint>
#include <cstdio>
#include <mutex>
#include <string>
#include <thread>
#include <vector>

#include <getopt.h>

static void print_usage(FILE* out, const char* const name) {
  fprintf(out, "Usage: %s [options]\n", name);
  fprintf(out, "option:\n");
  fprintf(out, "\t-t {number}\tthe number of threads (default is 8)\n");
  fprintf(out, "\t-n {number}\tthe number to add (default is 1,000,000\n");
  fprintf(out, "\t-l [semaphore|filter|cpp11]\tthe type of lock (default is semaphore)\n");
  fprintf(out, "\t-h\tprint this message\n");
}

enum class lock_type {
  semaphore, filter, cpp11
};

struct config {
  uint32_t number_of_threads;
  uint32_t number_to_add;
  ::lock_type lock_type;

  config()
      : number_of_threads(8),
      number_to_add(1000000),
      lock_type(::lock_type::semaphore) {
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
    if (opt_lock_type == "semaphore") {
      config.lock_type = ::lock_type::semaphore;
    } else if (opt_lock_type == "filter") {
      config.lock_type = ::lock_type::filter;
    } else if (opt_lock_type == "cpp11") {
      config.lock_type = ::lock_type::cpp11;
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
static std::mutex mutex;
void increase_with_cpp11(uint32_t n) {
  for (uint32_t i = 0; i < n; i += 1) {
    std::lock_guard<std::mutex> lock(mutex);
    g_count += 1;
  }
}

int main(int argc, char * const argv[]) {
  const auto&& config = parse_argument(argc, argv);

  std::vector<std::thread> threads;
  switch (config.lock_type) {
  case ::lock_type::cpp11: {
    std::mutex mutex;
    for (size_t i = 0; i < config.number_of_threads; i += 1) {
      threads.emplace_back(increase_with_cpp11, config.number_to_add / config.number_of_threads);
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

  printf("Count: %u\n", g_count);

  return 0;
}
