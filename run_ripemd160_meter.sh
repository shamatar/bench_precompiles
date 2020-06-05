#!/bin/sh
RAYON_NUM_THREADS=1 cargo test --release -- --nocapture --ignored benchmark_ripemd160_precompile

