#!/usr/bin/env -S just --justfile
# just (https://github.com/casey/just)
# cargo-sort (https://github.com/DevinR528/cargo-sort)

set windows-shell := ["powershell.exe", "-Command"]

list:
    just --list

fmt:
    cargo +nightly fmt
    cargo sort -w

clippy:
    cargo clippy

test *TEST:
    cmake -E env RUST_TEST_THREADS=1 cargo +nightly test --release {{ TEST }} -j 1

build:
    cargo build --release

doc:
    cargo doc

run CMD *OPTIONS:
    cargo run --release --bin {{ CMD }} -- {{ OPTIONS }}

check: fmt clippy test
