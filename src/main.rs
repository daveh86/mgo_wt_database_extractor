extern crate libc;
use libc::{c_int, size_t, c_void};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;


struct wt_connection;
struct wt_event_handler;
struct wt_session;
struct wt_cursor;


#[link(name = "wt_rust_wrap")]
extern {
    fn conn_open(home: *const c_char,
    errhandler: *mut wt_event_handler,
    config: *const c_char,
    conn: *mut *mut wt_connection) -> c_int;

    fn session_open(conn: *mut wt_connection,
    errhandler: *mut wt_event_handler,
    config: *const c_char,
    conn: *mut *mut wt_session) -> c_int;

    fn session_close(session: *mut wt_session,
        config: *const c_char) -> c_int;

    fn conn_close(conn: *mut wt_connection,
        config: *const c_char) -> c_int;

    fn create_table(session: *mut wt_session,
	name: *const c_char, config: *const c_char) -> c_int;

    fn drop_table(session: *mut wt_session,
        name: *const c_char, config: *const c_char) -> c_int;

    fn cursor_open(session: *mut wt_session,
	uri: *const c_char, to_dup: *mut wt_cursor, config : *const c_char,
	cursor: *mut *mut wt_cursor) -> c_int;

    fn cusror_close(cursor: *mut wt_cursor) -> c_int;

    // Cursor data manip
    fn cursor_get_value(cursor: *mut wt_cursor, value: *mut c_void) -> c_int;
    fn cursor_get_key(cursor: *mut wt_cursor, key: *mut c_void) -> c_int;
    fn cursor_set_value(cursor: *mut wt_cursor, value: *mut c_void) -> ();
    fn cursor_set_key(cursor: *mut wt_cursor, key: *mut c_void) -> ();

    // Cursor actions
    fn cursor_insert(cursor: *mut wt_cursor) -> c_int;
    fn cursor_next(cursor: *mut wt_cursor) -> c_int;
    fn cursor_perv(cursor: *mut wt_cursor) -> c_int;
    fn cursor_search(cursor: *mut wt_cursor) -> c_int;
    fn cursor_reset(cursor: *mut wt_cursor) -> c_int;
}

fn main() {
	// WT_CONN*
	let mut conn: *mut wt_connection = ptr::null_mut();

	// WT_SESSION*
	let mut session: *mut wt_session = ptr::null_mut();

	// Variables
	let home = CString::new("WT_TEST").unwrap();
	let conf = CString::new("create,statistics=(fast)").unwrap();

	unsafe {
		let res = conn_open(
			home.as_ptr(),
			ptr::null_mut(),
			conf.as_ptr(),
			&mut conn);
		
		session_open(conn,
			ptr::null_mut(),
			ptr::null_mut(),
			&mut session);

		session_close(session, ptr::null_mut());
		conn_close(conn, ptr::null_mut());
	}
	println!("Hello World!");
}

