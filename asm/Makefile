arch ?= $(shell uname -m)
cc ?= gcc

files := restore.o
paths := $(patsubst %, $(arch)/obj/%, $(files))

.PHONY: all

all: $(arch)/obj/liblrs_asm.a
	
$(arch)/obj/liblrs_asm.a: $(paths)
	@mkdir -p $(arch)/obj
	ar rcs $(arch)/obj/liblrs_asm.a $(paths)

$(arch)/obj/%.o: $(arch)/src/%.s
	@mkdir -p $(arch)/obj
	$(cc) -c -o "$@" "$<"
