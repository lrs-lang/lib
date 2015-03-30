#!/bin/sh

set -e

rustdoc -L target/debug/deps src/lib.rs
cp assets/doc_style.css doc/main.css
cd doc
rm *.woff
