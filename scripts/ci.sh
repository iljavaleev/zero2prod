#!/bin/bash

cargo clippy
cargo fmt 
cargo audit
cargo tarpaulin --ignore-tests
