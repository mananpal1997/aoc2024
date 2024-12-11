#!/bin/bash

set -eux

DAY=$1
PART=$2

if [[ -f "days/day${DAY}/tests/part${PART}_test.rs" ]]; then
    cargo test -p "day${DAY}" --test "part${PART}_test" -q
else
    echo "test file not found, skipping test..."
fi
cargo build -q -r
./target/release/aoc2024 --day $DAY --part $PART
