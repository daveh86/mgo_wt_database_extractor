extern crate libc;
use libc::{c_int, size_t};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;


struct wt_connection;
struct wt_event_handler;
struct wt_session;


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

