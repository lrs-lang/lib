srcs := $(shell find src/ -name "*.rs")
bins := $(patsubst src/%.rs, bin/%, $(srcs))

musl := "/home/julian/c/musl"

.PHONY: all clean lib

all: $(bins)

bin/%: src/%.rs ../obj/liblinux.rlib Makefile
	@mkdir -p "$(@D)"
	# rustc -O -C lto -C no-stack-check -L ../obj -C link-args="-nostdlib $(musl)/lib/crt1.o -L $(musl)/lib -static -l c" "$<" -o "$@"
	rustc -O -C lto -Z no-landing-pads -L ../obj -L /usr/local/lib "$<" -o "$@"
	# rustc -g -L ../obj "$<" -o "$@"

clean:
	rm bin/*
