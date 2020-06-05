#!/bin/sh
RAYON_NUM_THREADS=1 cargo test --release -- --nocapture --ignored benchmark_bn_add_precompile

