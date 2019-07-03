#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Test with coverage -----------------------------------------------------

log Testing code using unit tests and doc tests
cargo test --release

# --- Test with coverage -----------------------------------------------------

log Measuring code coverage with Tarpaulin
cargo tarpaulin --release -v --out Xml --ciserver travis-ci
