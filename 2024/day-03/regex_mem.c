#include <stdlib.h>
#include "./regex_mem.h"

regex_t* alloc_regex_t(void) {
    return malloc(sizeof (regex_t));
}

void free_regex_t(regex_t *regex) {
    free(regex);
}
