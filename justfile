#!/usr/bin/env -S just --justfile
# just (https://github.com/casey/just)
# cargo-sort (https://github.com/DevinR528/cargo-sort)

set windows-shell := ["cmd.exe", "/c"]

cargo-command-prefix := env_var_or_default("CARGO_COMMAND_PREFIX", "")

fmt:
	cargo +nightly {{cargo-command-prefix}} fmt
	cargo {{cargo-command-prefix}} sort -w

clippy:
	cargo {{cargo-command-prefix}} clippy

test +TEST="":
	cargo {{cargo-command-prefix}} test --release {{TEST}}

build:
	cargo {{cargo-command-prefix}} build --release

check: fmt clippy test
