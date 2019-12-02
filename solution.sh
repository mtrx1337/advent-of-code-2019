#!/bin/bash
# ./solution.sh /path/to/input/location
SOL_PATH=$1

cd $SOL_PATH
cargo build
time cat input.txt | cargo run
