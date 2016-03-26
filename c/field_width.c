#include <stdio.h>

int main(void) {
    printf("'%5d'\n", 1);
    printf("'%*d'\n", 5, 2);
    printf("'%-5d'\n", 3);
    printf("'%*d'\n", -5, 4);
    return 0;
}
