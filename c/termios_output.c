#include <stdio.h>
#include <termios.h>
#include <unistd.h>

#define print(FLAG) \
printf(#FLAG " (%04X): %s\n", FLAG, term.c_oflag & FLAG ? "true": "false");

int main() {
    struct termios term;

    if (tcgetattr(STDIN_FILENO, &term) < 0) {
        printf("can't get tty settings\n");
        return -1;
    }

    printf("%04X\n", term.c_oflag);
    print(OPOST);
    print(ONLCR);
    print(OCRNL);
    print(ONOCR);
    print(ONLRET);
    print(OFILL);
    print(OFDEL);
    print(NLDLY);
    print(CRDLY);
    print(TABDLY);
    print(BSDLY);
    print(VTDLY);
    print(FFDLY);

    print(XTABS);

    return 0;
}
