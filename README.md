Directory structure
-------------------

*lib* contains the wired tiger rust driver. 
It requires that you have downloaded, compiled and 
installed (`make install`) wiredtiger on your system:
http://source.wiredtiger.com/develop/build-posix.html

*src* contains the source code for our database clone application.
If everything works, it can be started with just:

    cargo build
    cargo run
