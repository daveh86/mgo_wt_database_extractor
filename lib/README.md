Commands
```
gcc -c -Wall -Werror -fpic wt_rust_wrap.c -I /home/david/work/wiredtiger/build_posix/
gcc -shared -o libwt_rust_wrap.so wt_rust_wrap.o -l wiredtiger -L /home/david/work/wiredtiger/build_posix/.libs/
gcc -static -o libwt_rust_wrap.a wt_rust_wrap.o -l wiredtiger -L /home/david/work/wiredtiger/build_posix/.libs/
```
