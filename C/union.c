#include <stdio.h>

union byte4 {
    int i;
    short s[2];
    char c[4];
};

int main(void) {
    union byte4 u;
    u.i = 0x12345678;

    printf("%X %X\n", u.s[0], u.s[1]); // 5678 1234
    printf("%X %X %X %X\n", u.c[0], u.c[1], u.c[2], u.c[3]); // 78 56 34 12

    return 0;
}

