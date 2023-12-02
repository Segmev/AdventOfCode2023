#!/bin/bash

echo "Creating new day $1"
cargo new "day$1"
touch "day$1/input.txt" day$1/test.txt
cp boilerplate/{part1.rs,part2.rs,main.rs} "day$1/src/."
ln ./utils.rs "day$1/src/utils.rs"