#include <errno.h>
#include <stdio.h>

int main(int argc, char *argv[])
{
  fopen("./asdasdasdasdas_no_exist_file", "r");
  perror("Error");
  return 0;
}
