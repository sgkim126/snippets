#include <getopt.h>
#include <stdio.h>

int main(int argc, char *argv[])
{
  int opt;
  while ((opt = getopt(argc, argv, "ab:c:d")) != -1) {
      switch (opt) {
        case 'a':
          printf("a\n");
          break;
        case 'b':
          printf("b:%s\n", optarg);
          break;
        case 'c':
          printf("c:%s\n", optarg);
          break;
        case 'd':
          printf("d\n");
          break;
          break;
        default:
          printf("%s [-a] [-b b] [-c c] [-d]\n", argv[0]);
          return -1;
      }
  }
  return 0;
}
