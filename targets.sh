#!/bin/sh

function trans {
    case "$1" in
        i686) echo "i686-unknown-linux-gnu" ;;
        x86_64) echo "x86_64-unknown-linux-gnu" ;;
        aarch64) echo "aarch64-unknown-linux-gnu" ;;
        arm) echo "arm-unknown-linux-gnueabi" ;;
        lkern-kernel) echo "aarch64-lkern-kernel" ;;
        *)
            echo "unknown target"
            exit 1
            ;;
    esac
}

function native {
    rustc -V -v | grep "^host" | cut -d' ' -f 2
}

function all {
    trans i686
    trans x86_64
    trans aarch64
    trans arm
    trans lkern-kernel
}

if [[ $# -eq 0 ]]; then
    echo "Usage: $0 <native|all|trans>"
    exit 1
fi

case "$1" in
    native) native;;
    all) all;;
    trans) trans $2;;
    *)
        echo "unknown command"
        exit 1
        ;;
esac
