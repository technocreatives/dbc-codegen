# CAN DBC code generator for Rust

Generates Rust messages from a `dbc` file.

⚠️ This is experimental - use with caution. Breaking changes will happen when you least expect it. ⚠️

## Usage

Generate `messages.rs` from `example.dbc`.
```bash
cargo run -- testing/dbc-examples/example.dbc dir/where/messages_rs/file/is/written
```

If some field name starts with a non-alphabetic character or is a Rust keyword then it is prepended with `x`.

For example:
```
VAL_ 512 Five 0 "0Off" 1 "1On" 2 "2Oner" 3 "3Onest";
```
..is generated to:
```rust
pub enum BarFive {
    X0off,
    X1on,
    X2oner,
    X3onest,
    Other(bool),
}
```

`Type` here:
```
SG_ Type : 30|1@0+ (1,0) [0|1] "boolean" Dolor
```
..would become Rust keyword `type` therefore it is prepended with `x`:
```rust
pub fn xtype(&self) -> BarType {
    match self.xtype_raw() {
        false => BarType::X0off,
        true => BarType::X1on,
        x => BarType::Other(x),
    }
}
```

## Development

```bash
# generate messages.rs
cargo run -- testing/dbc-examples/example.dbc testing/can-messages/src

# build all binaries
cargo build --all

# run all tests
cargo test --all

# format before commiting anything
cargo fmt --all
```

### Generate .kdc from .dbc

Use [canmatrix](https://github.com/ebroecker/canmatrix) if you need to generate a new `.kcd` file from a `.dbc`.

```bash
# https://canmatrix.readthedocs.io/en/latest/installation.html
pip install canmatrix
pip install git+https://github.com/ebroecker/canmatrix#egg=canmatrix[kcd]

# generate .kcd
canconvert testing/dbc-examples/example.dbc testing/dbc-examples/example.kcd
```
