srcs := $(shell find src/ -name "*.rs")
bins := $(patsubst src/%.rs, bin/%, $(srcs))

musl := "/home/julian/c/musl"

.PHONY: all clean lib

all: $(bins)

bin/%: src/%.rs ../obj/liblrs.rlib Makefile
	@mkdir -p "$(@D)"
	@# lrsc -O -C lto -C link-args="-nostdlib $(musl)/lib/crt1.o -L $(musl)/lib -static -l c" "$<" -o "$@"
	@# lrsc --target=i686-unknown-linux-gnu -C link-args="-nostdlib $(musl)/lib/crt1.o -L $(musl)/lib -static -l c" "$<" -o "$@"
	lrsc -O -C lto --target=i686-unknown-linux-gnu -C link-args="-nostdlib -static" "$<" -o "$@"
	@# lrsc -O -C lto "$<" -o "$@"
	@# lrsc -O -C lto "$<" --emit=llvm-ir
	@# lrsc -g "$<" -o "$@"

clean:
	rm bin/*
