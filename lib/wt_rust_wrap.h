#ifndef __RUST_WT_WRAP
#define __RUST_WT_WRAP
#include <wiredtiger.h>

int conn_open(char * home, WT_EVENT_HANDLER *event_handler, const char *config, WT_CONNECTION** conn);
int conn_close(WT_CONNECTION* conn, const char *config);
int session_open(WT_CONNECTION* conn, WT_EVENT_HANDLER *event_handler, const char *config, WT_SESSION **session);
int session_close(WT_SESSION* session, const char *config);
int create_table(WT_SESSION *session, const char *name, const char *config);
int drop_table(WT_SESSION *session, const char *name, const char *config);
int cursor_open(WT_SESSION* session, const char *uri, WT_CURSOR *to_dup, const char *config, WT_CURSOR **cursor);
int cursor_close(WT_CURSOR *cursor);
int cursor_get_value(WT_CURSOR *cursor, void **value);
int cursor_get_key(WT_CURSOR *cursor, void **key);
void cursor_set_value(WT_CURSOR *cursor, void *value);
void cursor_set_key(WT_CURSOR *cursor, void *key);
int cursor_insert(WT_CURSOR *cursor);
int cursor_next(WT_CURSOR *cursor);
int cursor_prev(WT_CURSOR *cursor);
int cursor_search(WT_CURSOR *cursor);
int cursor_reset(WT_CURSOR *cursor);
#endif //__RUST_WT_WRAP
