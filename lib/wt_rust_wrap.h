#ifndef __RUST_WT_WRAP
#define __RUST_WT_WRAP
#include <wiredtiger.h>

int conn_open(char * home, WT_EVENT_HANDLER *event_handler, const char *config, WT_CONNECTION** conn);
int conn_close(WT_CONNECTION* conn, const char *config);
int session_open(WT_CONNECTION* conn, WT_EVENT_HANDLER *event_handler, const char *config, WT_SESSION **session);
int session_close(WT_SESSION* session, const char *config);
#endif //__RUST_WT_WRAP
