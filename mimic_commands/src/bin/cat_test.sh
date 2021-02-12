#!/usr/bin/env bash
set -e

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
pj_root_dir="$script_dir/../../../"
cd "$pj_root_dir"

cargo build --release --bin cat
my_cat="./target/release/cat"
command diff <($my_cat invalid_file 2>&1) <(cat invalid_file 2>&1)
command diff <($my_cat Cargo.toml) <(cat Cargo.toml)
command diff <($my_cat Cargo.toml Cargo.lock) <(cat Cargo.toml Cargo.lock)
