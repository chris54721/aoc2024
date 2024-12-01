#!/bin/bash
set -e

[ -z "$1" ] && echo "Usage: ./makemyday.sh [day]"

DIR="day$1"
gsed -i '$ d' Cargo.toml
printf '    \"%s\",\n]' "$DIR" >> Cargo.toml
cargo new "$DIR"
gsed "s/%DIR%/$DIR/g" .base/main.rs > "$DIR/src/main.rs"
mkdir "$DIR/input"
touch "$DIR/input/input.txt"

curl "https://adventofcode.com/2024/day/$((10#$1))/input" -H "Cookie: session=$(<.aoc_session)" -o "$DIR/input/input.txt"

open "$DIR/src/main.rs"