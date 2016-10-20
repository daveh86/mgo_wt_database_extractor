#ifndef __RUST_WT_WRAP
#define __RUST_WT_WRAP
#include <wiredtiger.h>

// Meta item creation/destruction
int conn_open(char * home, WT_EVENT_HANDLER *event_handler, const char *config, WT_CONNECTION** conn);
int conn_close(WT_CONNECTION* conn, const char *config);
int session_open(WT_CONNECTION* conn, WT_EVENT_HANDLER *event_handler, const char *config, WT_SESSION **session);
int session_close(WT_SESSION* session, const char *config);
int cursor_open(WT_SESSION* session, const char *uri, WT_CURSOR *to_dup, const char *config, WT_CURSOR **cursor);
int cursor_close(WT_CURSOR *cursor);

// Session level ops
int create_table(WT_SESSION *session, const char *name, const char *config);
int drop_table(WT_SESSION *session, const char *name, const char *config);

// Cursor Get-ers
int cursor_get_value_i64(WT_CURSOR *cursor, int64_t *value);
int cursor_get_key_i64(WT_CURSOR *cursor, int64_t *key);
int cursor_get_value_str(WT_CURSOR *cursor, char **value);
int cursor_get_key_str(WT_CURSOR *cursor, char **key);
int cursor_get_value_item(WT_CURSOR *cursor, char **value, int *size);
int cursor_get_key_item(WT_CURSOR *cursor, char **key, int *size);

// Cursor Set-ers
void cursor_set_value(WT_CURSOR *cursor, void *value);
void cursor_set_key(WT_CURSOR *cursor, void *key);
void cursor_set_value_item(WT_CURSOR *cursor, char *value, int size);
void cursor_set_key_item(WT_CURSOR *cursor, char *key, int size);

// Cursor manip
int cursor_insert(WT_CURSOR *cursor);
int cursor_next(WT_CURSOR *cursor);
int cursor_prev(WT_CURSOR *cursor);
int cursor_search(WT_CURSOR *cursor);
int cursor_reset(WT_CURSOR *cursor);
#endif //__RUST_WT_WRAP
