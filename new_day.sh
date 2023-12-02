#!/bin/bash

echo "Creating new day $1"
cargo new "day$1"
touch "day$1/input.txt" day$1/test.txt
cp boilerplate/{utils.rs,part1.rs,part2.rs} "day$1/src/."