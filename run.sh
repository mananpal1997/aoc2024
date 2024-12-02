#!/bin/bash

set -eux

DAY=$1
PART=$2

cargo test -p "day${DAY}" --test "part${PART}_test" -q
cargo build -q -r
./target/release/aoc2024 --day $DAY --part $PART
