#!/bin/sh

set -e

# rustdoc -L obj src/lrs/lib.rs
./rustdoc -L obj src/lrs/lib.rs
cd ../doc
./lrs_doc
# cp assets/doc_style.css doc/main.css
# cd doc
# rm *.woff
