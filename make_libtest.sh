#!/bin/bash

if [[ $# -eq 0 ]]; then
    target="$(./targets.sh native)"
else
    target="$(./targets.sh trans $1)"
fi

lrsc --out-dir obj/$target --target $target src/test/lib.rs
