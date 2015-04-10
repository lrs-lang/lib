#!/bin/sh

function build {
    rustc --emit=link --out-dir=obj -L obj src/$1/lib.rs
}

OBJS="arch core sys dev fs time_base process poll file time_ext dir user_group linux"

mkdir -p obj

for obj in $OBJS; do
    echo "  building $obj"
    build $obj
done
