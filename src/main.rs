extern crate libc;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;


struct WtConnection;
struct WtEventHandler;
struct WtSession;
struct WtCursor;


#[link(name = "wt_rust_wrap")]
extern {
    fn conn_open(home: *const c_char,
				 errhandler: *mut WtEventHandler,
				 config: *const c_char,
				 conn: *mut *mut WtConnection) -> c_int;

    fn session_open(conn: *mut WtConnection,
					errhandler: *mut WtEventHandler,
					config: *const c_char,
					conn: *mut *mut WtSession) -> c_int;

    fn session_close(session: *mut WtSession,
					 config: *const c_char) -> c_int;

    fn conn_close(conn: *mut WtConnection,
				  config: *const c_char) -> c_int;

    fn create_table(session: *mut WtSession,
					name: *const c_char, config: *const c_char) -> c_int;

    fn drop_table(session: *mut WtSession,
				  name: *const c_char, config: *const c_char) -> c_int;

    fn cursor_open(session: *mut WtSession,
				   uri: *const c_char, to_dup: *mut WtCursor, config : *const c_char,
				   cursor: *mut *mut WtCursor) -> c_int;

    fn cursor_close(cursor: *mut WtCursor) -> c_int;

    // Cursor data manip
    fn cursor_get_value(cursor: *mut WtCursor, value: *mut c_void) -> c_int;
    fn cursor_get_key(cursor: *mut WtCursor, key: *mut c_void) -> c_int;
    fn cursor_set_value(cursor: *mut WtCursor, value: *mut c_void) -> ();
    fn cursor_set_key(cursor: *mut WtCursor, key: *mut c_void) -> ();

    // Cursor actions
    fn cursor_insert(cursor: *mut WtCursor) -> c_int;
    fn cursor_next(cursor: *mut WtCursor) -> c_int;
    fn cursor_perv(cursor: *mut WtCursor) -> c_int;
    fn cursor_search(cursor: *mut WtCursor) -> c_int;
    fn cursor_reset(cursor: *mut WtCursor) -> c_int;
}

fn main() {
	// WT_CONN*
	let mut conn: *mut WtConnection = ptr::null_mut();

	// WT_SESSION*
	let mut session: *mut WtSession = ptr::null_mut();

	// Variables
	let home = CString::new("WT_TEST").unwrap();
	let conf = CString::new("create,statistics=(fast)").unwrap();
	let table_name = CString::new("table:mytable").unwrap();
	let table_conf = CString::new("key_format=Q,value_format=S").unwrap();
	let mut cursor: *mut WtCursor = ptr::null_mut();
	let mut x: i64 = 123;
	let x_raw = &mut x as *mut i64;

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

		create_table(session, table_name.as_ptr(), table_conf.as_ptr());

		cursor_open(session,
					table_name.as_ptr(),
					ptr::null_mut(),
					ptr::null(),
					&mut cursor);
		cursor_set_key(cursor, x_raw as *mut c_void);
		cursor_set_value(cursor, table_conf.as_ptr() as *mut c_void);
		cursor_insert(cursor);
		cursor_close(cursor);
		session_close(session, ptr::null_mut());
		conn_close(conn, ptr::null_mut());
	}
	println!("Hello World!");
}

