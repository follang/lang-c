#ifndef PARC_CORPUS_TYPE_ENV_B_API_H
#define PARC_CORPUS_TYPE_ENV_B_API_H

typedef struct item item_t;
typedef int (*item_iter_cb)(item_t *item, void *ctx);

struct item {
    int id;
    const char *name;
    item_iter_cb next_cb;
    item_t *next;
};

typedef item_t *item_handle_t;

typedef struct item_bucket {
    item_handle_t slots[4];
    item_iter_cb callbacks[2];
} item_bucket;

int iterate_bucket(item_bucket *bucket, item_iter_cb cb);

#endif
