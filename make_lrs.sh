#!/bin/sh

target="$(./targets.sh native)"

function build {
    lrsc --out-dir=obj/$target src/$1/lib.rs
}

OBJS="$(cat LRSBuild1 | grep '^obj ' | cut '-d ' -f 2) lrs"

mkdir -p obj

for obj in $OBJS; do
    echo "  building $obj"
    build $obj
done
