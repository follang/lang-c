#ifndef PARC_CORPUS_COMBO_API_H
#define PARC_CORPUS_COMBO_API_H

#if !defined(COMBO_WIDE_BUILD)
#  error "config/base.h must be included before api.h"
#endif

typedef enum combo_mode {
    COMBO_MODE_SMALL = COMBO_ENUM_VALUE(1, 0),
    COMBO_MODE_LARGE = COMBO_ENUM_VALUE(1, 1)
} combo_mode;

typedef struct combo_config {
    combo_word_t mask;
    size_t count;
    combo_mode mode;
} combo_config;

COMBO_API combo_handle *combo_open(const combo_config *cfg);
COMBO_API int combo_log(combo_handle *handle, const char *fmt, va_list ap);

#endif
