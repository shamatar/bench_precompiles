#!/bin/sh
for D in $(find ./vectors -mindepth 0 -maxdepth 3 -name '*.csv') ; do
    rm $D ;
done

for D in $(find ./vectors -mindepth 0 -maxdepth 2 -name '*.json') ; do
    rm $D ;
done

cargo test --release --no-run
sleep 30
cargo test --release -- --nocapture --test-threads=1 generate_

