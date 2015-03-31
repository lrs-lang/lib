#!/bin/sh

set -e

rustdoc -L obj src/linux/lib.rs
cp assets/doc_style.css doc/main.css
cd doc
rm *.woff
