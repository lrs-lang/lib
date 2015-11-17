#!/bin/sh

function trans {
    case "$1" in
        i686) echo "i686-unknown-linux-gnu" ;;
        x86_64) echo "x86_64-unknown-linux-gnu" ;;
        aarch64) echo "aarch64-unknown-linux-gnu" ;;
        arm) echo "arm-unknown-linux-gnueabi" ;;
        *)
            echo "unknown target"
            exit 1
            ;;
    esac
}

function native {
    case "$(uname -m)" in
        i386 | i486 | i686 | x86) trans i686 ;;
        x86_64 | x86-64 | amd64) trans x86_64 ;;
        arm64 | aarch64) trans aarch64 ;;
        arm) trans arm ;;
        *)
            echo "unknown target"
            exit 1
            ;;
    esac
}

function all {
    trans i686
    trans x86_64
    trans aarch64
    trans arm
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
