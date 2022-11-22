This (temporary) repo is a repro which
produces an executable that segfaults at start, before reaching the main()

Steps to reproduce:

with LXD:

./runme

with docker:

docker build . -t litir
docker run litir

this will start gdb, type "r" in it, and you will be greeted with SIGSEGV feedback


