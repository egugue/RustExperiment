#!/usr/bin/env bash
set -e

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
pj_root_dir="$script_dir/../../../"
cd "$pj_root_dir"

cargo build --release --bin wc
my_wc="./target/release/wc"
command diff <($my_wc invalid_file 2>&1) <(wc invalid_file 2>&1)
command diff <($my_wc Cargo.lock) <(wc Cargo.lock)
