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
    cargo +nightly test --release {{ TEST }}

build:
    cargo build --release

doc:
    cargo doc

run CMD *OPTIONS:
    cargo run --release --bin {{ CMD }} -- {{ OPTIONS }}

check: fmt clippy test
