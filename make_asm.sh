#!/bin/sh

cd asm
if [[ ! -e target ]]; then
    ln -s $(uname -m) target
fi
make
