#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>

static struct termios original;

static void reset_input_mode() {
    tcsetattr(STDIN_FILENO, TCSANOW, &original);
}

static int reset_input_mode_at_exit() {
    if (tcgetattr(STDIN_FILENO, &original) < 0) {
        printf("can't get tty settings\n");
        return -1;
    }
    atexit(reset_input_mode);
}

int main() {
    struct termios term;

    if (reset_input_mode_at_exit() < 0) {
        return -1;
    }

    if (tcgetattr(STDIN_FILENO, &term) < 0) {
        printf("can't get tty settings\n");
        return -1;
    }

    term.c_oflag &= ~ONLCR;

    if (tcsetattr(STDIN_FILENO, TCSANOW, &term) < 0) {
        printf("can't set tty settings\n");
        return -1;
    }

    printf("A\nB\r\nC\r\n");

    return 0;
}
