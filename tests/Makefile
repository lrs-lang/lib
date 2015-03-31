srcs := $(shell find src/ -name "*.rs")
bins := $(patsubst src/%.rs, bin/%, $(srcs))

.PHONY: all clean lib

all: $(bins)

bin/%: src/%.rs ../obj/liblinux.rlib
	@mkdir -p "$(@D)"
	rustc -L ../obj "$<" -o "$@"

clean:
	rm bin/*
