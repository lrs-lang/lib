#!/bin/sh

next=$(($(cat last) + 1))
echo $next > last
echo -n "Bug name: "
read -e name
file=$(printf "open/%04d-%s.adoc" $next "${name// /-}")
echo "= $name" > "$file"
$EDITOR "$file"
