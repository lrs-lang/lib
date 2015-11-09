#!/bin/sh

mkdir -p obj
rustc -o obj/liblrs_core_plugin.so src/core_plugin/lib.rs
