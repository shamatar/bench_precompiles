#!/bin/sh
cargo test --release --no-run
sleep 30
cargo test --release -- --nocapture --test-threads=1 try_

