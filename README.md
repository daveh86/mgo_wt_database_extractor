What's really stored in your MongoDB WiredTiger [data directory](https://docs.mongodb.com/manual/reference/glossary/#term-data-directory), your `mongod --dbpath`? 

- Learn what's there and how to extract entire [namespaces](https://docs.mongodb.com/manual/reference/glossary/#term-namespace) including their associated [collections](https://docs.mongodb.com/manual/reference/glossary/#term-collection) and [indexes](https://docs.mongodb.com/manual/reference/glossary/#term-index)
- Consolidate multiple data directories into a large one or atomise a large data directory into many smaller ones
- Requires no running `mongod`
- Automatically fixes the `_mdb_catalog.wt` and `sizeStorer.wt` system databases so you can start running a [mongod](https://www.mongodb.com/download-center?jmp=nav#community) and verify with any downstream `mongo shell`, [Compass](https://www.mongodb.com/download-center?jmp=nav#compass) or [your favorite MongoDB tools](https://docs.mongodb.com/ecosystem/tools/administration-interfaces/)


Directory structure
-------------------

*lib* contains the wired tiger [Rust](https://www.rust-lang.org/) driver. 
It requires that you have downloaded, compiled and 
installed (`make install`) wiredtiger on your system:
http://source.wiredtiger.com/develop/build-posix.html

NB: If you intend to open existing MongoDB databases, you will need to 
ensure you build wiredtiger with [snappy](https://github.com/google/snappy) and use a WiredTiger branch for your MongoDB version:

    # macOS, or 'sudo apt-get install snappy' on some linux distros
    brew install snappy
    
    # Build and install WiredTiger
    git clone git@github.com:wiredtiger/wiredtiger.git
    cd wiredtiger
    git checkout mongodb-3.4
    ./configure --enable-strict --with-builtins=snappy,zlib --enable-verbose
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


Disclaimer
----------

This software is not supported by [MongoDB, Inc.](http://www.mongodb.com) under any of their commercial support subscriptions or otherwise. Any usage of this tool is at your own risk.
Bug reports, feature requests and questions can be posted in the [Issues](https://github.com/daveh86/mgo_wt_database_extractor/issues?state=open) section here on github.
