#! /usr/bin/env bash
set -ex

FILE=${1:-example.dbc}
cargo run --no-default-features --features "bitvec-backend" -- dbc-examples/$FILE can-messages-bitvec/src/
