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
