#!/bin/sh
RAYON_NUM_THREADS=4 cargo test --release -- --nocapture --ignored benchmark_go_sha256_precompile

