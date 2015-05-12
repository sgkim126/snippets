#if defined(WIN32) || defined(__MINGW32__)

#include <windows.h>
static inline void yield()
{
    Sleep(0);
}

#elif defined(__APPLE__) || defined(__linux__)

#include <sched.h>
static inline void yield()
{
    sched_yield();
}

#else

#error This platform is not supported.

#endif


static inline void memory_fence(void)
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
    while (sp->is_set) {
        yield();
    }

    sp->item = item;
    memory_fence();
    sp->is_set = 1;
}

int remove(sitem_t* sp)
{
    int item;

    while (!sp->is_set) {
        yield();
    }

    item = sp->item;
    memory_fence();
    sp->is_set = 0;

    return item;
}
