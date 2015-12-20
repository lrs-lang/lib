#!/bin/sh

set -e

target="$(./targets.sh native)"

rustdoc -w json -L obj/$target src/lrs/lib.rs
lrs_doc
cp assets/doc_style.css doc/style.css
