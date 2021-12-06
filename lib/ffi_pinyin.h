#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PinyinStr {
  uintptr_t len;
  char *data;
  bool owned;
} PinyinStr;

typedef struct PinyinArray {
  uintptr_t len;
  struct PinyinStr *array;
} PinyinArray;

char *plain(const char *str, int is_convert);

char *tone(const char *str, int is_convert);

char *letter(const char *str);

struct PinyinArray *plain_array(const char *str, int is_convert);

struct PinyinArray *tone_array(const char *str, int is_convert);

char *tone_multi(const char *str);

void free_pointer(char *ptr);

void free_array(struct PinyinArray *array);
