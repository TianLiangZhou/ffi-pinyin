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

char *plain(const char *str, int is_convert, int is_multi);

char *tone(const char *str, int is_convert, int is_multi);

char *tone_num(const char *str, int is_convert, int is_multi);

char *tone_num_end(const char *str, int is_convert, int is_multi);

char *letter(const char *str, int is_convert, int is_multi);

PinyinArray *plain_array(const char *str, int is_convert, int is_multi);

PinyinArray *tone_array(const char *str, int is_convert, int is_multi);

PinyinArray *tone_num_array(const char *str, int is_convert, int is_multi);

PinyinArray *tone_num_end_array(const char *str, int is_convert, int is_multi);

PinyinArray *letter_array(const char *str, int is_convert, int is_multi);

void free_pointer(char *ptr);

void free_array(PinyinArray *array);
