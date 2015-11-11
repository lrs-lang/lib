#!/bin/sh

set -e

rustdoc -w json -L obj src/lrs/lib.rs
lrs_doc
cp assets/doc_style.css doc/style.css
