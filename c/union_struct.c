#include <stdio.h>
#include <stdint.h>

typedef union union_t {
  int8_t i8;
  int16_t i16;
  int32_t i32;
  int64_t i64;
} myUnion;

typedef struct struct_t {
  int8_t i8;
  int16_t i16;
  int32_t i32;
  int64_t i64;
} myStruct;

int main (void) {
  myUnion u;
  myStruct s;

  printf("Address\n");
  printf("\n");
  printf("Union's i8 = %p, i16 = %p, i32 = %p, i64 = %p\n", &(u.i8), &(u.i16), &(u.i32), &(u.i64));
  printf("\n");
  printf("Struct's i8 = %p, i16 = %p, i32 = %p, i64 = %p\n", &(s.i8), &(s.i16), &(s.i32), &(s.i64));
  printf("-------------\n");
  printf("Union & Struct's Size\n");
  printf("\n");
  printf("myUnion's size = %zu\n", sizeof(myUnion));
  printf("myStruct's size = %zu\n", sizeof(myStruct));
  printf("-------------\n");
  printf("Sizes of types\n");
  printf("\n");
  printf("i8 = %zu, i16 = %zu, i32 = %zu, i64 = %zu\n", sizeof(int8_t), sizeof(int16_t),sizeof(int32_t), sizeof(int64_t));
  return 0;  
}
