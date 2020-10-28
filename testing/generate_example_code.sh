#! /usr/bin/env bash
set -ex

FILE=${1:-example.dbc}
cargo run -- dbc-examples/$FILE can-messages/src/
