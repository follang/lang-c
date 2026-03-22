#ifndef PARC_CORPUS_FEATURE_H
#define PARC_CORPUS_FEATURE_H

#include <stdarg.h>
#include <sys/types.h>
#include <unistd.h>

#if !defined(CORPUS_WIDE_ENV)
#  error "base.h must be included first"
#endif

#define CORPUS_HAVE_FORMAT 1

typedef off_t corpus_offset_t;

#endif
