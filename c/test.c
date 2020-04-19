#include <stdio.h>

int main(void) {
  int x = 0;
  int* ptr1 = &x;
  printf("ptr1's addr: %d = x's addr: %d, ptr1's AND %d, ptr1's value: %d\n",
         ptr1, &x, &ptr1, *ptr1);

  return 0;
}