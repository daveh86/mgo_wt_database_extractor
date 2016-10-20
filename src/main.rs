extern crate bson;
extern crate libc;
extern crate getopts;

use bson::{Bson, decode_document};
use getopts::Options;
use libc::c_void;
use std::env;
use std::ffi::CString;
use std::ffi::CStr;
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
                 conn: *mut *mut WtConnection) -> i32;

    fn session_open(conn: *mut WtConnection,
                    errhandler: *mut WtEventHandler,
                    config: *const c_char,
                    conn: *mut *mut WtSession) -> i32;

    fn session_close(session: *mut WtSession,
                     config: *const c_char) -> i32;

    fn conn_close(conn: *mut WtConnection,
                  config: *const c_char) -> i32;

    fn create_table(session: *mut WtSession,
                    name: *const c_char, config: *const c_char) -> i32;

/*
    fn drop_table(session: *mut WtSession,
                  name: *const c_char, config: *const c_char) -> i32;
*/
    fn cursor_open(session: *mut WtSession,
                   uri: *const c_char, to_dup: *mut WtCursor, config : *const c_char,
                   cursor: *mut *mut WtCursor) -> i32;

    fn cursor_close(cursor: *mut WtCursor) -> i32;

    // Cursor data manip
    fn cursor_get_key_i64(cursor: *mut WtCursor, key: *mut i64) -> i32;
//    fn cursor_get_key_str(cursor: *mut WtCursor, key: *mut *mut c_char) -> i32;
    fn cursor_get_value_str(cursor: *mut WtCursor, value: *mut *mut c_char) -> i32;
    fn cursor_get_value_item(cursor: *mut WtCursor, value: *mut *mut u8, sz: *mut usize) -> i32;
//    fn cursor_get_key_item(cursor: *mut WtCursor, key: *mut *mut u8, sz: *mut usize) -> i32;
//    fn cursor_set_value(cursor: *mut WtCursor, value: *mut c_void) -> ();
    fn cursor_set_key(cursor: *mut WtCursor, key: *mut c_void) -> ();
    fn cursor_set_value_item(cursor: *mut WtCursor, value: *mut u8, sz: usize) -> ();
    fn cursor_set_key_item(cursor: *mut WtCursor, key: *mut u8, sz: usize) -> ();


    // Cursor actions
//    fn cursor_insert(cursor: *mut WtCursor) -> i32;
    fn cursor_next(cursor: *mut WtCursor) -> i32;
//    fn cursor_perv(cursor: *mut WtCursor) -> i32;
    fn cursor_search(cursor: *mut WtCursor) -> i32;
//    fn cursor_reset(cursor: *mut WtCursor) -> i32;
}

fn wt_err(code: i32) -> i32 {
    match code {
        0 => 0,
        -31800 => panic!("WT_ROLLBACK"),
        -31801 => panic!("WT_DUPLICATE_KEY"),
        -31802 => panic!("WT_ERROR"),
        -31803 => -31803, // WT_NOTFOUND
        -31804 => panic!("WT_PANIC"),
        -31805 => panic!("WT_RESTART"),
        -31806 => panic!("WT_RUN_RECOVERY"),
        -31807 => panic!("WT_CACHE_FULL"),
        2 => panic!("WT_OPEN FAIL"),
        _ => 1,
    };
    return code;
}

fn get_tablenames(session: *mut WtSession, wanted: String) -> Vec<String> {
    // WT_CURSOR*
    let mut cursor: *mut WtCursor = ptr::null_mut();

    // Variables
    let table_name = CString::new("table:_mdb_catalog").unwrap();
    let wanted_table = CString::new(wanted.clone()).unwrap();

    let mut refetched_value: *mut u8 = ptr::null_mut();
    let mut refetched_len: usize = 0;

    let mut vec :Vec<String> = Vec::new();

    unsafe {
        wt_err(cursor_open(session,
            table_name.as_ptr(),
            ptr::null_mut(),
            ptr::null(),
            &mut cursor));

        while wt_err(cursor_next(cursor)) == 0 {
            cursor_get_value_item(cursor, &mut refetched_value, &mut refetched_len);
            let slicey = slice::from_raw_parts(refetched_value, refetched_len);
            let doc = decode_document(&mut Cursor::new(slicey.to_vec())).unwrap();
            let file = doc.get("ident");
            if file != None {
                let ns = match doc.get("ns").unwrap() {
                    &Bson::String(ref s) => s.clone(),
                    _ => String::new(),
                };

                println!("collection {} is file {}", ns, file.unwrap());
                if ns == wanted {
                        let out = match file.unwrap() {
                        &Bson::String(ref s) => s.clone(),
                        _ => String::new(),
                    };
                    vec.push(out);
                    if doc.get("idxIdent") != None {
                        let idxdoc = doc.get("idxIdent").unwrap();
                        for (k,v) in idxdoc.to_json().as_object().unwrap() {
                            let str = v.to_string().replace("\"","");
                            vec.push(str);
                        }
                    }
                }
            }
        }
        cursor_close(cursor);
    }
    return vec;
}

fn get_metadata(session: *mut WtSession, wanted: String) -> String {
    // WT_CURSOR*
    let mut cursor: *mut WtCursor = ptr::null_mut();

    // Variables
    let table_name = CString::new("metadata:").unwrap();
    let wanted_table = CString::new(wanted).unwrap();

    let mut key : *mut c_char = ptr::null_mut();
    unsafe {
        wt_err(cursor_open(session,
            table_name.as_ptr(),
            ptr::null_mut(),
            ptr::null(),
            &mut cursor));

        cursor_set_key(cursor, wanted_table.as_ptr() as *mut c_void);

        if cursor_search(cursor) == 0 {
            wt_err(cursor_get_value_str(cursor, &mut key));
            return CStr::from_ptr(key).to_string_lossy().into_owned();
        }
    }
    return String::new();
}

fn list_tables(session: *mut WtSession) -> () {

    // WT_CURSOR*
    let mut cursor: *mut WtCursor = ptr::null_mut();

    // Variables
    let table_name = CString::new("table:_mdb_catalog").unwrap();

    //let mut x: i64 = 123;
    //let x_raw = &mut x as *mut i64;
    let mut refetched_key: i64 = 0;
    let mut refetched_value: *mut u8 = ptr::null_mut();
    let mut refetched_len: usize = 0;
    unsafe {
        // TODO: Error handling? https://doc.rust-lang.org/book/error-handling.html
        wt_err(cursor_open(session,
            table_name.as_ptr(),
            ptr::null_mut(),
            ptr::null(),
            &mut cursor));

        while wt_err(cursor_next(cursor)) == 0 {
            wt_err(cursor_get_key_i64(cursor, &mut refetched_key));
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-l] [options]", program);
    print!("{}", opts.usage(&brief));
}

fn copy_table(src_session: *mut WtSession, out_path: String, table_name: String) -> i32 {
    let out_conf = CString::new("create,statistics=(fast)").unwrap();

    let mut dest_conn: *mut WtConnection = ptr::null_mut();
    let mut dest_session: *mut WtSession = ptr::null_mut();
    let mut src_cursor: *mut WtCursor = ptr::null_mut();
    let mut dest_cursor: *mut WtCursor = ptr::null_mut();
    let wt_table_name = CString::new("table:".to_string() + &table_name).unwrap();

    unsafe {
        // Acquire resources
        let out_path_cstr = CString::new(out_path).unwrap();
        wt_err(conn_open(out_path_cstr.as_ptr(),
                         ptr::null_mut(),
                         out_conf.as_ptr(),
                         &mut dest_conn));

        wt_err(session_open(dest_conn,
                            ptr::null_mut(),
                            ptr::null_mut(),
                            &mut dest_session));

        wt_err(cursor_open(src_session,
                           wt_table_name.as_ptr(),
                           ptr::null_mut(),
                           ptr::null(),
                           &mut src_cursor));

        // Grab table_config from db_path and use it to create_table in out_path
        let table_config = get_metadata(src_session, "table:".to_string() + &table_name);
        create_table(dest_session,
                     wt_table_name.as_ptr(),
                     table_config.as_ptr() as *mut i8);

        // Open cursor on our new table
        wt_err(cursor_open(dest_session,
                           wt_table_name.as_ptr(),
                           ptr::null_mut(),
                           ptr::null(),
                           &mut dest_cursor));

        // Copy the data
        let mut refetched_key: i64 = 0;
        let mut refetched_value: *mut u8 = ptr::null_mut();
        let mut refetched_len: usize = 0;
        while wt_err(cursor_next(src_cursor)) == 0 {

            // Fetch the data from db_path
            wt_err(cursor_get_key_i64(src_cursor, &mut refetched_key));
            cursor_get_value_item(src_cursor, &mut refetched_value, &mut refetched_len);

            // TODO: Store the data in out_path
//            wt_err(cursor_set_key_i64(dest_cursor, &mut refetched_key));
//            cursor_set_value_item(dest_cursor, &mut refetched_value, &mut refetched_len);
        }

        // Cleanup resources in reverse order to how we acquired them
        // TODO: What happens if we exit early due to an error, do we need to clean up?
        cursor_close(dest_cursor);
        cursor_close(src_cursor);
        session_close(dest_session, ptr::null_mut());
        conn_close(dest_conn, ptr::null_mut());
    }
    return 0;
}

//fn fix_destination_metadata(source: *mut WtSession, destination: String) -> i32 {
//    return 0;
//}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("d", "dbpath", "set dbpath to read from (Defaults to /data/db)", "DBPATH");
    opts.optopt("o", "outpath", "set dbpath to write to", "OUTPATH");
    opts.optopt("t", "tables", "list of tables to be copied", "TABLES");
    opts.optflag("l", "list", "list the table mappings");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let db_path = match matches.opt_str("d") {
        Some(s) => CString::new(s).unwrap(),
        None => CString::new("/data/db").unwrap(),
    };
    let out_path = matches.opt_str("o");
    let tables = matches.opt_str("t");

    // WT_CONN*
    let mut conn: *mut WtConnection = ptr::null_mut();
    // WT_SESSION*
    let mut session: *mut WtSession = ptr::null_mut();
    let db_conf = CString::new("create,statistics=(fast)").unwrap();

    unsafe {
        wt_err(conn_open(
            db_path.as_ptr(),
            ptr::null_mut(),
            db_conf.as_ptr(),
            &mut conn));

        wt_err(session_open(conn,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut session));

        if matches.opt_present("l") {
            list_tables(session);
            let want = String::from("table:_mdb_catalog");
            println!("{}", get_metadata(session, want));
            println!("{:?}", get_tablenames(session, String::from("test.test")));

        } else {
            if out_path == None {
                println!("No Outpath set!");
                return();
            }
            if tables == None {
                println!("No tables listed!");
                return();
            }

            let wt_out_path = match matches.opt_str("o") {
                Some(s) => s,
                None => panic!("No Outpath set!"),
            };
            let table_input = match tables {
                Some(s) => s,
                None => String::new(),
            };
            println!("{}", table_input.clone());
            let table_list = get_tablenames(session, table_input);
            for table_name in table_list.clone() {
                println!("{}", table_name);
            }
            println!("{}", wt_out_path);
            for table_name in table_list {
                copy_table(session, wt_out_path.clone(), table_name);
//                fix_destination_metadata(session, wt_out_path);
            }
            println!("do something else");
        }
        session_close(session, ptr::null_mut());
        conn_close(conn, ptr::null_mut());
    }
}

