# CAN DBC code generator for Rust

Generates Rust messages from a `dbc` file.

⚠️ This is experimental - use with caution. Breaking changes will happen when you least expect it. ⚠️

# Usage

Generate `messages.rs` from `example.dbc`.
```bash
cargo run -- testing/dbc-examples/example.dbc dir/where/messages_rs/file/is/written
```

# Development

```bash
# generate messages.rs
cargo run -- testing/dbc-examples/example.dbc testing/can-messages/src

# build all binaries
cargo build --all

# run all tests
cargo test --all
```

## Generate .kdc from .dbc

Use [canmatrix](https://github.com/ebroecker/canmatrix) if you need to generate a new `.kcd` file from a `.dbc`.

```bash
# https://canmatrix.readthedocs.io/en/latest/installation.html
pip install canmatrix
pip install git+https://github.com/ebroecker/canmatrix#egg=canmatrix[kcd]

# generate .kcd
canconvert testing/dbc-examples/example.dbc testing/dbc-examples/example.kcd
```
