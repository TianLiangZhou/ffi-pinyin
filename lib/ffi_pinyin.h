#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

char *plain(const char *str);

char *tone(const char *str);

char *letter(const char *str);

char *tone_multi(const char *str);

void free_pointer(char *ptr);
