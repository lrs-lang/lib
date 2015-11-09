#!/bin/sh

function build {
    lrsc --out-dir=obj -L obj src/$1/lib.rs
}

OBJS="$(cat LRSBuild | grep '^obj ' | cut '-d ' -f 2) lrs"

mkdir -p obj

for obj in $OBJS; do
    echo "  building $obj"
    build $obj
done
