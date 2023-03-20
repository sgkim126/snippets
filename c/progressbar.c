#include <stdio.h>
#include <unistd.h>

void display_progress_bar(int progress) {
    static const int PROGRESS_BAR_LENGTH = 20;
    int filled = progress * PROGRESS_BAR_LENGTH / 100;

    printf("\r[");
    for (int i = 0; i < filled; i += 1) {
        printf("#");
    }
    for (int i = filled; i < PROGRESS_BAR_LENGTH; i += 1) {
        printf(" ");
    }
    printf("] %3d%%", progress);
    fflush(stdout);
}

int main() {
    for (int progress = 0; progress <= 100; progress += 1) {
        usleep(100000);
        display_progress_bar(progress);
    }
    printf("\n");
    return 0;
}

