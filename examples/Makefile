srcs := $(shell find src/ -name "*.rs")
bins := $(patsubst src/%.rs, bin/%, $(srcs))

musl := "/home/julian/c/musl"
ops := -L ../obj

.PHONY: all clean lib

all: $(bins)

bin/%: src/%.rs ../obj/liblrs.rlib Makefile
	@mkdir -p "$(@D)"
	@# lrsc -O -C lto $(ops) -C link-args="-nostdlib $(musl)/lib/crt1.o -L $(musl)/lib -static -l c" "$<" -o "$@"
	@# lrsc -O -C lto $(ops) "$<" -o "$@"
	@# lrsc -O -C lto $(ops) "$<" --emit=llvm-ir
	lrsc -g $(ops) "$<" -o "$@"

clean:
	rm bin/*
