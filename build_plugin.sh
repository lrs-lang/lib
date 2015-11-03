#!/bin/sh

mkdir -p obj
/usr/local/bin/rustc -o obj/liblrs_core_plugin.so src/core_plugin/lib.rs
