all:
	gcc -c -o regex_mem.o regex_mem.c
	ar crs libregex_mem.a regex_mem.o
	zig build test
	zig build run
