#!/usr/bin/env bash
set -e

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
pj_root_dir="$script_dir/../../../"
cd "$pj_root_dir"

cargo build --release --bin tail
my_tail="./target/release/tail"

#[ "$($my_tail < Cargo.lock)" = "$(tail < Cargo.lock)" ]
#[ "$($my_tail invalid_file 2>&1)" = "$(tail invalid_file 2>&1)" ]
[ "$($my_tail Cargo.lock)" = "$(tail Cargo.lock)" ]
[ "$($my_tail Cargo.toml)" = "$(tail Cargo.toml)" ]
#[ "$($my_tail Cargo.lock Cargo.toml)" = "$(tail Cargo.lock Cargo.toml)" ]

echo "done"
