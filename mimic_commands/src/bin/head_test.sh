#!/usr/bin/env bash
set -e

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
pj_root_dir="$script_dir/../../../"
cd "$pj_root_dir"

cargo build --release --bin head
my_head="./target/release/head"

#[ "$($my_head < Cargo.lock)" = "$(head < Cargo.lock)" ]
#[ "$($my_head invalid_file 2>&1)" = "$(head invalid_file 2>&1)" ]
[ "$($my_head Cargo.lock)" = "$(head Cargo.lock)" ]
[ "$($my_head Cargo.toml)" = "$(head Cargo.toml)" ]
[ "$($my_head Cargo.lock Cargo.toml)" = "$(head Cargo.lock Cargo.toml)" ]
#[ "$(echo 'ああaa' | $my_head)" = "$(echo 'ああaa' | head)" ]

echo "done"