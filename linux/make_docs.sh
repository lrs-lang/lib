#!/bin/sh

set -e

cargo doc
cp assets/doc_style.css target/doc/main.css
cd target/doc
rm *.woff
