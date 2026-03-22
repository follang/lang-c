#ifndef PARC_CORPUS_API_H
#define PARC_CORPUS_API_H

typedef struct corpus_config {
    int CORPUS_FIELD(flags);
    corpus_offset_t CORPUS_FIELD(offset);
} corpus_config;

typedef struct corpus_handle_s *corpus_handle_t;

CORPUS_API corpus_handle_t CORPUS_CALL corpus_open(const corpus_config *cfg);

#if CORPUS_HAVE_FORMAT
CORPUS_API int CORPUS_CALL corpus_format(const char *fmt, va_list ap);
#endif

#endif
