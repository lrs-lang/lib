asm_files := restore.o
crt_files := start.o start_thread.o stop_thread.o

cc ?= clang

.PHONY: all clean

define GEN

all: $(1)/obj/liblrs_asm.a $(1)/obj/liblrs_crt.a

$(1)_asm_paths := $$(patsubst %, $(1)/obj/%, $(asm_files))
$(1)_crt_paths := $$(patsubst %, $(1)/obj/%, $(crt_files))

$(1)/obj:
	mkdir $(1)/obj

$(1)/obj/%.o: $(1)/src/%.s | $(1)/obj
	$(cc) --target=$(1) -c -o "$$@" "$$<"

$(1)/obj/liblrs_asm.a: $$($(1)_asm_paths)
	ar rcs $(1)/obj/liblrs_asm.a $$($(1)_asm_paths)

$(1)/obj/liblrs_crt.a: $$($(1)_crt_paths)
	ar rcs $(1)/obj/liblrs_crt.a $$($(1)_crt_paths)

clean: $(1)_clean

.PHONY: $(1)_clean

$(1)_clean:
	rm -r $(1)/obj

endef

$(eval $(call GEN,x86_64-unknown-linux-gnu))
$(eval $(call GEN,i686-unknown-linux-gnu))
$(eval $(call GEN,aarch64-unknown-linux-gnu))
$(eval $(call GEN,armv6-unknown-linux-gnueabi))
