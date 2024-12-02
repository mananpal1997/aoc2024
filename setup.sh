#!/bin/bash

set -eux

DAY=$1

pushd days
cargo new day$DAY --lib
pushd day$DAY
cat ../../lib.template > src/lib.rs
mkdir -p tests input
touch src/part1.rs src/part2.rs
cat ../../part.template > src/part1.rs
cat ../../part.template > src/part2.rs
touch input/part1.txt input/part2.txt
touch tests/part1_test.rs tests/part2_test.rs
cat ../../test.template > tests/part1_test.rs
cat ../../test.template > tests/part2_test.rs
sed -i "s/<n>/$DAY/g" tests/part1_test.rs
sed -i "s/<part>/1/g" tests/part1_test.rs
sed -i "s/<n>/$DAY/g" tests/part2_test.rs
sed -i "s/<part>/2/g" tests/part2_test.rs
popd
popd

python3 helper.py $DAY