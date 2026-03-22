#ifndef PARC_CORPUS_BASE_H
#define PARC_CORPUS_BASE_H

#define CORPUS_API extern
#define CORPUS_CALL
#define CORPUS_FIELD(name) name

#if __SIZEOF_POINTER__ >= 8
#  define CORPUS_WIDE_ENV 1
#else
#  define CORPUS_WIDE_ENV 0
#endif

#endif
