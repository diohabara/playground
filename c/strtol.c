#include <stdio.h>
#include <stdlib.h>

int main(void) 
{
  char *number10 = "+255";
  char* endptr;
  long x;
  x = strtol(number10, &endptr, 10);
  printf("number10 = %s, x = %ld\n, endptr = %s\n", number10, x, endptr);
}
