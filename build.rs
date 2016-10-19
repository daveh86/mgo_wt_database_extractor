use std::env;
use std::process::Command;


fn main() {
    let p = env::current_dir().unwrap();
    println!("cargo:rustc-link-search={}/lib", p.display());

    // Remove two files, then run two build scripts to recompile the C-API bridge in lib
    Command::new("rm").arg("lib/wt_rust_wrap.o").output()
        .expect("removes wt_rust_wrap.o");
    Command::new("rm").arg("lib/libwt_rust_wrap.so").output()
        .expect("removes libwt_rust_wrap.so");

    // gcc -c -Wall -Werror -fpic wt_rust_wrap.c
    Command::new("gcc")
        .arg("-c")
        .arg("-Wall")
        .arg("-Werror")
        .arg("-fpic")
        .arg("lib/wt_rust_wrap.c")
        .arg("-o")
        .arg("lib/wt_rust_wrap.o")
        .output()
        .expect("gcc should compile the wt_rust_wrap object file");

    // gcc -shared -o libwt_rust_wrap.so wt_rust_wrap.o -l wiredtiger
    Command::new("gcc")
        .arg("-shared")
        .arg("-o")
        .arg("lib/libwt_rust_wrap.so")
        .arg("lib/wt_rust_wrap.o")
        .arg("-l")
        .arg("wiredtiger")
        .output()
        .expect("gcc should compile the libwt_rust_wrap shared object file");
}
