srcs := $(shell find src/ -name "*.rs")
bins := $(patsubst src/%.rs, bin/%, $(srcs))

musl := "/home/julian/c/musl"
ops := -C no-stack-check -Z no-landing-pads -L ../obj

.PHONY: all clean lib

all: $(bins)

bin/%: src/%.rs ../obj/liblrs.rlib Makefile
	@mkdir -p "$(@D)"
	@# rustc -O -C lto $(ops) -C link-args="-nostdlib $(musl)/lib/crt1.o -L $(musl)/lib -static -l c" "$<" -o "$@"
	rustc -O -C lto $(ops) "$<" -o "$@"
	@# rustc -O -C lto $(ops) "$<" --emit=asm
	@# rustc -g $(ops) "$<" -o "$@"

clean:
	rm bin/*
