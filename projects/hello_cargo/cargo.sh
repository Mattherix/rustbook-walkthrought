#!/bin/sh
cargo check
cargo build
cargo run
cargo build --release
