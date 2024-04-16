#!/usr/bin/env bash
set -e

# We want the code to panic if there is an integer overflow
export RUSTFLAGS="-C overflow-checks=on"

cargo test --release -p jf-utils #-- -Zunstable-options --report-time
cargo test --release -p jf-plonk --lib --bins #-- -Zunstable-options --report-time
cargo test --release -p jf-primitives --features "test-srs, gadgets" #-- -Zunstable-options --report-time # enable test-srs feature for gen_srs_for_testing
cargo test --release -p jf-primitives-core #-- -Zunstable-options --report-time
cargo test --release -p jf-merkle-tree --features gadgets #-- -Zunstable-options --report-time
cargo test --release -p jf-pcs --features test-srs #-- -Zunstable-options --report-time
cargo test --release -p jf-rescue --features gadgets #-- -Zunstable-options --report-time
cargo test --release -p jf-signature --features "bls, schnorr, gadgets" #-- -Zunstable-options --report-time
cargo test --release -p jf-vdf #-- -Zunstable-options --report-time
cargo test --release -p jf-vid --features test-srs #-- -Zunstable-options --report-time
