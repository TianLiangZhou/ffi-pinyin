#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Mode {
  Plain = 1,
  Tone = 2,
  Letter = 3,
  ToneNum = 4,
  ToneNumEnd = 5,
};
typedef uint8_t Mode;

typedef struct PinyinStr {
  uintptr_t len;
  char *data;
  bool owned;
  uint8_t convert;
} PinyinStr;

typedef struct PinyinArray {
  uintptr_t len;
  struct PinyinStr *array;
} PinyinArray;

char *to_pinyin(const char *str,
                int is_ignore_unknown_char,
                int is_multi,
                unsigned char separator,
                int not_split_unknown_char,
                Mode mode,
                int is_slug);

struct PinyinArray *to_pinyin_array(const char *str,
                                    int is_ignore_unknown_char,
                                    int is_multi,
                                    int not_split_unknown_char,
                                    Mode mode);

void free_pointer(char *ptr);

void free_array(struct PinyinArray *array);
