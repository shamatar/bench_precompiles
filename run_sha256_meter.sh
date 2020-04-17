#!/bin/sh
RAYON_NUM_THREADS=4 cargo test --release -- --nocapture --ignored benchmark_sha256_precompile

