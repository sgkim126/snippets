#include <signal.h>
#include <stdio.h>

void signal_handler (int signum) {
    printf("%d\n", signum);
}

int main (void) {
    signal (SIGHUP, signal_handler);
    signal (SIGINT, signal_handler);
    signal (SIGQUIT, signal_handler);
    signal (SIGTERM, signal_handler);
    signal (SIGKILL, signal_handler);

    while (1);
}
