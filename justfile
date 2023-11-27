#!/usr/bin/env -S just --justfile
# just (https://github.com/casey/just)

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

list:
    just --list

# cargo-sort (https://github.com/DevinR528/cargo-sort)
fmt:
    cargo +nightly fmt
    cargo sort -w
    just --fmt --unstable

clippy:
    cargo clippy

[unix]
test *TEST:
    /usr/bin/env RUST_TEST_THREADS=1 cargo +nightly test --release -j 1 {{ TEST }}

[windows]
test *TEST:
    #!powershell -NoLogo
    $ENV:RUST_TEST_THREADS = "1"
    cargo +nightly test --release -j 1 {{ TEST }}

build:
    cargo build --release

build-wasm:
    cmake -E env CC="$WASI_SDK_PATH/bin/clang --sysroot=$WASI_SDK_PATH/share/wasi-sysroot" cargo build --target wasm32-wasi --bin bff-gui

doc:
    cargo doc

run CMD *OPTIONS:
    cargo run --release --bin {{ CMD }} -- {{ OPTIONS }}

install:
    cargo install --path bff-cli --bin bff-cli
    cargo install --path bff-gui --bin bff-gui

install-dev-deps:
    rustup install nightly
    rustup update nightly
    cargo install cargo-sort
    cargo install flamegraph
    {{ if os() == 'windows' { 'cargo install blondie' } else { '' } }}

# flamegraph (https://github.com/flamegraph-rs/flamegraph)
[unix]
flamegraph CMD *OPTIONS:
    /usr/bin/env CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root --release --bin {{ CMD }} -- -- {{ OPTIONS }}

# flamegraph (https://github.com/flamegraph-rs/flamegraph) and blondie (https://github.com/nico-abram/blondie)
[windows]
flamegraph CMD *OPTIONS:
    #!powershell -NoLogo
    $ENV:CARGO_PROFILE_RELEASE_DEBUG = "true"
    $ENV:DTRACE = "blondie_dtrace"
    cargo flamegraph --release --bin {{ CMD }} -- -- {{ OPTIONS }}

check: fmt clippy test
