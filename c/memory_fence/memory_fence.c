#if defined(__GNUC__)

#define INLINE inline __attribute__((__always_inline__))
#define LIKELY(x) __builtin_expect(!!(x), 1)
#define UNLIKELY(x) __builtin_expect(!!(x), 0)

#elif defined(_MSC_VER)

#define INLINE __forceinline
#define LIKELY(x) (x)
#define UNLIKELY(x) (x)

#else

#define INLINE inline
#define LIKELY(x) (x)
#define UNLIKELY(x) (x)

#endif


#if defined(_WIN32_WINNT) && _WIN32_WINNT >= 0x0400

#if _WIN32_WINNT > 0x0602 // Windows 8
#include <Processthreadsapi.h>
#else
#define WIN32_LEAN_NO_MEAN
#define NOMINMAX
#include <Windows.h>
#undef NOMINMAX
#undef WIN32_LEAN_NO_MEAN
#endif

static INLINE void yield()
{
    SwitchToThread();
}

#elif defined(__APPLE__) || defined(__linux__)

#include <sched.h>
static INLINE void yield()
{
    sched_yield();
}

#else

#error This platform is not supported.

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

typedef struct {
    int item;
    int is_set;
} sitem_t;

void insert(sitem_t* sp, int item)
{
    if (UNLIKELY(sp->is_set)) {
        while (LIKELY(sp->is_set)) {
            yield();
        }
    }

    sp->item = item;
    memory_fence();
    sp->is_set = 1;
}

int remove(sitem_t* sp)
{
    int item;

    if (UNLIKELY(!sp->is_set)) {
        while (LIKELY(!sp->is_set)) {
            yield();
        }
    }

    item = sp->item;
    memory_fence();
    sp->is_set = 0;

    return item;
}
