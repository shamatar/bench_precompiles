#!/bin/sh
RAYON_NUM_THREADS=1 cargo test --release -- --nocapture --ignored benchmark_keccak_sponge_price

