#ifndef PARC_CORPUS_COMBO_BASE_H
#define PARC_CORPUS_COMBO_BASE_H

#include <stddef.h>
#include <stdint.h>
#include <stdarg.h>

#define COMBO_API extern
#define COMBO_ENUM_VALUE(base, delta) ((base) + (delta))

#if __SIZEOF_POINTER__ >= 8 && __SIZEOF_SHORT__ == 2
#  define COMBO_WIDE_BUILD 1
#else
#  define COMBO_WIDE_BUILD 0
#endif

#if COMBO_WIDE_BUILD
typedef uint64_t combo_word_t;
#else
typedef uint32_t combo_word_t;
#endif

typedef struct combo_handle combo_handle;

#endif
