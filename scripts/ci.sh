#!/bin/bash
cargo sqlx prepare
cargo clippy

cargo fmt 

cargo audit

cargo tarpaulin --ignore-tests
