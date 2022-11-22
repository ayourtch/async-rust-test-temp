This (temporary) repo is a repro which
produces an executable that segfaults at start, before reaching the main()

Steps to reproduce:

with LXD:

./runme

with docker:

docker build . -t litir
docker run litir

this will start gdb, type "r" in it, and you will be greeted with SIGSEGV feedback

printing backtrace:

```

(gdb) r
Starting program: /async-rust-test-temp/target/debug/litir 
warning: Error disabling address space randomization: Operation not permitted

Program received signal SIGSEGV, Segmentation fault.
0x0000000000068806 in ?? ()
(gdb) bt
#0  0x0000000000068806 in ?? ()
#1  0x00007fab42551bb6 in ossl_init () at curl/lib/vtls/openssl.c:1551
#2  0x00007fab4254dd6d in Curl_ssl_init () at curl/lib/vtls/vtls.c:250
#3  0x00007fab42548cb7 in global_init (flags=3, memoryfuncs=true) at curl/lib/easy.c:168
#4  0x00007fab42548d0d in curl_global_init (flags=3) at curl/lib/easy.c:230
#5  0x00007fab425487cf in curl::init::{closure#0} () at src/lib.rs:103
#6  0x00007fab42548a52 in std::sync::once::{impl#4}::call_once::{closure#0}<curl::init::{closure_env#0}> ()
    at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sync/once.rs:276
#7  0x00007fab4240ce0d in std::sync::once::Once::call_inner () at library/std/src/sync/once.rs:434
#8  0x00007fab425489ae in std::sync::once::Once::call_once<curl::init::{closure_env#0}> (
    self=0x7fab42a9c470 <curl::init::INIT>, f=...)
    at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sync/once.rs:276
#9  0x00007fab425487b5 in curl::init () at src/lib.rs:96
#10 0x00007fab4240e1d6 in curl::INIT_CTOR::init_ctor () at src/lib.rs:147
#11 0x00007fab428e3dc3 in libc_start_init () at ../src_musl/src/env/__libc_start_main.c:64
#12 0x00007fab428e3de8 in libc_start_main_stage2 () at ../src_musl/src/env/__libc_start_main.c:91
#13 0x00007fab4240ed23 in _start ()
(gdb) 
```
