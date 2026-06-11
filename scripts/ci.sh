#!/bin/bash
cargo sqlx prepare -- --all-targets --all-features
cargo clippy

cargo fmt 

cargo audit

cargo tarpaulin --ignore-tests
