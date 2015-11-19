.PHONY: all clean gdb

-include config.mk

all: tests

-include tests.d

tests:
	lrsc --test --emit=link,dep-info $(RFLAGS) lib.rs

gdb: tests
	gdb -x gdb.conf --args tests $(test)

clean:
	rm -f tests tests.d
