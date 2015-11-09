arch ?= $(shell uname -m)
cc ?= clang

asm_files := restore.o
asm_paths := $(patsubst %, target/obj/%, $(asm_files))

crt_files := start.o
crt_paths := $(patsubst %, target/obj/%, $(crt_files))

.PHONY: all

all: target/obj/liblrs_asm.a target/obj/liblrs_crt.a

target:
	ln -s $(arch) target

target/src/%: target
	@#

target/obj/%.o: target/src/%.s
	@mkdir -p target/obj
	$(cc) -c -o "$@" "$<"

target/obj/liblrs_asm.a: $(asm_paths)
	@mkdir -p target/obj
	ar rcs target/obj/liblrs_asm.a $(asm_paths)

target/obj/liblrs_crt.a: $(crt_paths)
	@mkdir -p target/obj
	ar rcs target/obj/liblrs_crt.a $(crt_paths)
