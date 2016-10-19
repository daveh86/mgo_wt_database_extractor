#include "wt_rust_wrap.h"
int conn_open(char * home, WT_EVENT_HANDLER *event_handler, const char *config, WT_CONNECTION** conn)
{
	return(wiredtiger_open(home, event_handler, config, conn));
	
}

int conn_close(WT_CONNECTION* conn, const char *config)
{
	return (conn->close(conn, config));
}

int session_open(WT_CONNECTION* conn, WT_EVENT_HANDLER *event_handler, const char *config, WT_SESSION **session)
{
	return (conn->open_session(conn, event_handler, config, session));
}

int session_close(WT_SESSION* session, const char *config) 
{
	return (session->close(session, config));
}
