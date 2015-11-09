cc ?= clang

asm_files := restore.o
asm_paths := $(patsubst %, target/obj/%, $(asm_files))

crt_files := start.o
crt_paths := $(patsubst %, target/obj/%, $(crt_files))

.PHONY: all clean

all: target/obj/liblrs_asm.a target/obj/liblrs_crt.a

target/obj:
	mkdir target/obj

target/obj/%.o: target/src/%.s | target/obj
	$(cc) $(CFLAGS) -c -o "$@" "$<"

target/obj/liblrs_asm.a: $(asm_paths)
	ar rcs target/obj/liblrs_asm.a $(asm_paths)

target/obj/liblrs_crt.a: $(crt_paths)
	ar rcs target/obj/liblrs_crt.a $(crt_paths)

clean:
	rm -rf target/obj
