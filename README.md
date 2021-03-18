# CAN DBC code generator for Rust

Generates Rust messages from a `dbc` file.

⚠️ This is experimental - use with caution. Breaking changes will happen when you least expect it. ⚠️

## Installation

With cargo:

```bash
cargo install dbc-codegen --git https://github.com/technocreatives/dbc-codegen --branch main
```

## Usage

Generate `messages.rs` from `example.dbc`:

```bash
dbc-codegen testing/dbc-examples/example.dbc dir/where/messages_rs/file/is/written
```

### Field/variant rename rules

If some field name starts with a non-alphabetic character or is a Rust keyword then it is prefixed with `x`.

For example:

```
VAL_ 512 Five 0 "0Off" 1 "1On" 2 "2Oner" 3 "3Onest";
```

…is generated as:

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

…conflicts with the Rust keyword `type`. Therefore we prefix it with `x`:

```rust
pub fn xtype(&self) -> BarType {
    match self.xtype_raw() {
        false => BarType::X0off,
        true => BarType::X1on,
        x => BarType::Other(x),
    }
}
```

## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
