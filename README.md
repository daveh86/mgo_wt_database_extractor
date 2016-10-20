Directory structure
-------------------

*lib* contains the wired tiger [Rust](https://www.rust-lang.org/) driver. 
It requires that you have downloaded, compiled and 
installed (`make install`) wiredtiger on your system:
http://source.wiredtiger.com/develop/build-posix.html

NB: If you intend to open existing MongoDB databases, you will need to 
ensure you build wiredtiger with snappy:

    # macOS, or 'sudo apt-get install snappy' on some linux distros
    brew install snappy
    
    # Build and install WiredTiger
    git clone git@github.com:wiredtiger/wiredtiger.git
    cd wiredtiger
    ./configure --with-spinlock=gcc --enable-strict CFLAGS=-ggdb --with-builtins=snappy
    make
    make install


*src* contains the source code for our database clone application.
If everything works, it can be started with just:

    cargo build
    cargo run


Usage
-----

```
cargo run -- -h
   Compiling wt_ffi v0.1.0 (file:///Users/pzrq/Projects/wiredtiger_rust_app)
    Finished debug [unoptimized + debuginfo] target(s) in 1.67 secs
     Running `target/debug/wt_ffi -h`
Usage: target/debug/wt_ffi [-l] [options]

Options:
    -d, --dbpath DBPATH set dbpath to read from (Defaults to /data/db)
    -o, --outpath OUTPATH
                        set dbpath to write to
    -n, --namespaces NAMESPACES
                        space-separated list of namespaces to be copied
    -l, --list          list the table mappings
    -h, --help          print this help menu
```
   
    
Examples
--------

```
# List table mappings
cargo run -- -l

# Copy zips.small_zips and zips.zips 
# from default dbpath /data/db to outpath WT_TEST
cargo run -- -n "zips.small_zips zips.zips" -o WT_TEST
```
