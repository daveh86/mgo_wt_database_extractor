extern crate bson;
extern crate libc;
use bson::decode_document;
use libc::{c_int, c_void};
use std::ffi::CString;
//use std::ffi::CStr;
use std::io::Cursor;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

enum WtConnection {}
enum WtEventHandler {}
enum WtSession {}
enum WtCursor {}


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
/*
    fn create_table(session: *mut WtSession,
					name: *const c_char, config: *const c_char) -> c_int;

    fn drop_table(session: *mut WtSession,
				  name: *const c_char, config: *const c_char) -> c_int;
*/
    fn cursor_open(session: *mut WtSession,
				   uri: *const c_char, to_dup: *mut WtCursor, config : *const c_char,
				   cursor: *mut *mut WtCursor) -> c_int;

    fn cursor_close(cursor: *mut WtCursor) -> c_int;

    // Cursor data manip
    fn cursor_get_key_i64(cursor: *mut WtCursor, key: *mut i64) -> c_int;
//    fn cursor_get_value_str(cursor: *mut WtCursor, value: *mut *mut c_char) -> c_int;
    fn cursor_get_value_item(cursor: *mut WtCursor, value: *mut *mut u8, sz: *mut usize) -> c_int;
//    fn cursor_get_key_item(cursor: *mut WtCursor, key: *mut *mut u8, sz: *mut usize) -> c_int;
//    fn cursor_set_value(cursor: *mut WtCursor, value: *mut c_void) -> ();
//    fn cursor_set_key(cursor: *mut WtCursor, key: *mut c_void) -> ();
     

    // Cursor actions
//    fn cursor_insert(cursor: *mut WtCursor) -> c_int;
    fn cursor_next(cursor: *mut WtCursor) -> c_int;
//    fn cursor_perv(cursor: *mut WtCursor) -> c_int;
//    fn cursor_search(cursor: *mut WtCursor) -> c_int;
//    fn cursor_reset(cursor: *mut WtCursor) -> c_int;
}

fn list_tables(session: *mut WtSession) -> () {

        // WT_CURSOR*
        let mut cursor: *mut WtCursor = ptr::null_mut();

        // Variables
        let table_name = CString::new("table:_mdb_catalog").unwrap();

        //let table_conf = CString::new("key_format=q,value_format=S").unwrap();
        //let mut x: i64 = 123;
        //let x_raw = &mut x as *mut i64;
        let mut refetched_key: i64 = 0;
        let mut refetched_value: *mut u8 = ptr::null_mut();
        let mut refetched_len: usize = 0;
        unsafe {
                // TODO: Error handling? https://doc.rust-lang.org/book/error-handling.html
                let ret = cursor_open(session,
                        table_name.as_ptr(),
                        ptr::null_mut(),
                        ptr::null(),
                        &mut cursor);

                if ret != 0 {
                        println!("Error. opening connection!");
                        return();
                }
                while cursor_next(cursor) == 0 {
                        let ret = cursor_get_key_i64(cursor, &mut refetched_key);
			if ret != 0 {
				continue;
			}
                        cursor_get_value_item(cursor, &mut refetched_value, &mut refetched_len);
                        let slicey = slice::from_raw_parts(refetched_value, refetched_len);
                        let doc = decode_document(&mut Cursor::new(slicey.to_vec())).unwrap();
                        let file = doc.get("ident");
                        if file != None {
                                let ns = doc.get("ns").unwrap();
                                println!("collection {} is file {}", ns, file.unwrap());
                                if doc.get("idxIdent") != None {
                                        let idxdoc = doc.get("idxIdent").unwrap();
					println!("indexes:");
                                        for (k,v) in idxdoc.to_json().as_object().unwrap() {
                                                println!("\t{} : {}", k, v);
                                        }
                                }
				println!("")
                        }
                }
                cursor_close(cursor);
	}
}

fn main() {
	// WT_CONN*
	let mut conn: *mut WtConnection = ptr::null_mut();
	// WT_SESSION*
        let mut session: *mut WtSession = ptr::null_mut();
	
	let db_path = CString::new("/data/db").unwrap();
        let db_conf = CString::new("create,statistics=(fast)").unwrap();

	unsafe {
		let ret = conn_open(
                        db_path.as_ptr(),
                        ptr::null_mut(),
                        db_conf.as_ptr(),
                        &mut conn);
                if ret != 0 {
                        println!("Error. opening connection!");
                        return();
                }

                let ret = session_open(conn,
                        ptr::null_mut(),
                        ptr::null_mut(),
                        &mut session);

                if ret != 0 {
                        println!("Error. opening connection!");
                        return();
                }
		list_tables(session);
		session_close(session, ptr::null_mut());
		conn_close(conn, ptr::null_mut());
	}
}

