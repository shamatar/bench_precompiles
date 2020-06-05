#!/bin/sh
cargo test --release --no-run
sleep 10
cargo test --release -- --nocapture --test-threads=1 try_for_sha256

