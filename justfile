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
    cargo +nightly clippy --tests

check:
    cargo +nightly check --tests

deny:
    cargo deny check

test *TEST:
    cargo +nightly test --release -- {{ TEST }}

build CMD:
    cargo build --bin {{ CMD }}

build-release CMD:
    cargo build --release --bin {{ CMD }}

doc:
    cargo doc

run CMD *OPTIONS:
    cargo run --bin {{ CMD }} -- {{ OPTIONS }}

run-release CMD *OPTIONS:
    cargo run --release --bin {{ CMD }} -- {{ OPTIONS }}

install:
    cargo install --path bff-cli --bin bff-cli

install-dev-deps:
    rustup install nightly
    rustup update nightly
    cargo install --locked cargo-sort flamegraph cargo-deny zizmor cargo-machete
    cargo install --locked --git https://github.com/rust-lang/measureme summarize
    {{ if os() == 'windows' { 'cargo install --locked blondie' } else { '' } }}

# flamegraph (https://github.com/flamegraph-rs/flamegraph)
[unix]
flamegraph CMD *OPTIONS:
    /usr/bin/env CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root --release --bin {{ CMD }} -- {{ OPTIONS }}

# flamegraph (https://github.com/flamegraph-rs/flamegraph) and blondie (https://github.com/nico-abram/blondie)
[windows]
flamegraph CMD *OPTIONS:
    #!powershell -NoLogo
    $ENV:CARGO_PROFILE_RELEASE_DEBUG = "true"
    $ENV:DTRACE = "blondie_dtrace"
    cargo flamegraph --release --bin {{ CMD }} -- {{ OPTIONS }}

# cargo-build timings (https://doc.rust-lang.org/cargo/reference/timings.html),

# rustc -Ztime-passes, and measureme summarize (https://github.com/rust-lang/measureme)
[unix]
profile-compile *TARGET:
    #! /usr/bin/env bash
    set -euo pipefail
    out=".compile-profile"
    tmp_target="$(mktemp -d -t bff-self-profile-XXXXXX)"
    trap 'rm -rf "$tmp_target"' EXIT
    rm -rf "$out"
    mkdir -p "$out"
    CARGO_TARGET_DIR="$tmp_target" cargo +nightly build --timings {{ TARGET }}
    if [ -d "$tmp_target/cargo-timings" ]; then
      cp -f "$tmp_target"/cargo-timings/*.html "$out"/ || true
    fi
    CARGO_TARGET_DIR="$tmp_target" RUSTFLAGS="-Ztime-passes" cargo +nightly build -j 1 {{ TARGET }} 2> "$out/time-passes.log"
    grep -E "expand|typeck|borrowck|codegen|LLVM|link" "$out/time-passes.log" > "$out/time-passes-phases.txt" || true
    CARGO_TARGET_DIR="$tmp_target" RUSTFLAGS="-Zself-profile=$tmp_target/self-profile" cargo +nightly build {{ TARGET }}
    latest="$(find "$tmp_target" -name '*.events' -type f | head -n 1 || true)"
    if [ -n "$latest" ] && command -v summarize >/dev/null 2>&1; then
      prefix="${latest%.events}"
      summarize summarize "$prefix" > "$out/self-profile-summary.txt" || true
    elif [ -n "$latest" ]; then
      echo "summarize tool not found; install with: cargo install --locked --git https://github.com/rust-lang/measureme summarize" > "$out/self-profile-summary.txt"
    fi
    echo "Wrote compile profile artifacts to $out"

# cargo-build timings (https://doc.rust-lang.org/cargo/reference/timings.html),

# rustc -Ztime-passes, and measureme summarize (https://github.com/rust-lang/measureme)
[windows]
profile-compile *TARGET:
    #!powershell -NoLogo
    $out = ".compile-profile"
    $tmpTarget = Join-Path $env:TEMP ("bff-self-profile-" + [guid]::NewGuid().ToString("N"))
    if (Test-Path $out) { Remove-Item -Recurse -Force $out -ErrorAction SilentlyContinue }
    try {
        New-Item -ItemType Directory -Path $out -Force | Out-Null
        $env:CARGO_TARGET_DIR = $tmpTarget
        cargo +nightly build --timings {{ TARGET }}
        if (Test-Path "$tmpTarget\\cargo-timings") {
            Copy-Item "$tmpTarget\\cargo-timings\\*.html" -Destination $out -Force
        }
        $env:RUSTFLAGS = "-Ztime-passes"
        cargo +nightly build -j 1 {{ TARGET }} 2> "$out\\time-passes.log"
        Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue
        Select-String -Path "$out\\time-passes.log" -Pattern "expand|typeck|borrowck|codegen|LLVM|link" | Set-Content "$out\\time-passes-phases.txt"
        $env:RUSTFLAGS = "-Zself-profile=$tmpTarget\\self-profile"
        cargo +nightly build {{ TARGET }}
        Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue
        Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
        $latest = Get-ChildItem -Recurse -File $tmpTarget -Filter "*.events" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
        if ($latest -and (Get-Command summarize -ErrorAction SilentlyContinue)) {
            $prefix = [System.IO.Path]::Combine($latest.DirectoryName, [System.IO.Path]::GetFileNameWithoutExtension($latest.Name))
            summarize summarize $prefix | Set-Content "$out\\self-profile-summary.txt"
        } elseif ($latest) {
            Set-Content "$out\\self-profile-summary.txt" "summarize tool not found; install with: cargo install --locked --git https://github.com/rust-lang/measureme summarize"
        }
    } finally {
        Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue
        Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
        if (Test-Path $tmpTarget) { Remove-Item -Recurse -Force $tmpTarget -ErrorAction SilentlyContinue }
    }
    Write-Output "Wrote compile profile artifacts to $out"

zizmor:
    zizmor --persona auditor --collect all -- .github/workflows/build-wasm.yml .github/workflows/build.yml .github/workflows/nightly-release.yml .github/workflows/release.yml

machete:
    cargo machete

unused:
    cargo workspace-unused-pub

flint: fmt clippy check deny zizmor machete

clean:
    cargo clean
    cargo +nightly clean gc
