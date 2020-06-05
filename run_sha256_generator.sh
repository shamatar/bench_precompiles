#!/bin/sh
cd vectors/sha256/
cd current
rm -rf *
cd ..
cd proposed
rm -rf *
cd ..
cd ../..
cargo test --release --no-run
sleep 10
cargo test --release -- --test-threads=1 generate_for_sha256_current_pricing
sleep 10
cargo test --release -- --test-threads=1 generate_for_sha256_proposed_pricing

