# CAN DBC code generator for Rust

Generates Rust messages from a `dbc` file.

⚠️ This is experimental - use with caution. Breaking changes will happen when you least expect it. ⚠️

## Installation

Install published version using cargo
(assumes working installation of `cargo` and `rustc`):

```bash
cargo install dbc-codegen-cli
```

Install latest version from the repository,
also using cargo:

```bash
cargo install dbc-codegen-cli --git https://github.com/technocreatives/dbc-codegen --branch main
```

## Using dbc-codegen

Generate `messages.rs` from `example.dbc` using the CLI:

```bash
dbc-codegen testing/dbc-examples/example.dbc dir/where/messages_rs/file/is/written
```

Or put something like this into your `build.rs` file:

```rust
fn main() {
    let dbc_path = "../dbc-examples/example.dbc";
    let dbc_file = std::fs::read(dbc_path).unwrap();
    println!("cargo:rerun-if-changed={}", dbc_path);
    
    let mut out = std::io::BufWriter::new(std::fs::File::create("src/messages.rs").unwrap());
    dbc_codegen::codegen("example.dbc", &dbc_file, &mut out, true).unwrap();
}
```

## Using resulting Rust code

dbc-codegen generates Rust code,
that is expected to be in a cargo project.

All you need to add is the following dependencies:
e.g. by adding this to your `Cargo.toml`:

```toml
bitvec = { version = "0.21", default-features = false }
float-cmp = "0.8"
# Optional dependency, required only if you want to use arbitrary
# message implementations using the `arb` feature.
arbitrary = "1.0"
```

To use the code, add `mod messages` to your `lib.rs` (or `main.rs`).
You will most likely want to interact with the generated
`Messages` enum, and call `Messages::from_can_message(id, &payload)`.

Note: The generated code contains a lot of documentation!
Give `cargo doc --open` a try.

### Feature flags

The following (optional) features to be specified:

- `debug`: enables `Debug` derive
- `range_checked`: adds range checks in setters
- `arb`: Enables implementation of [`Arbitrary`] trait.
  Also requires you add `arbitrary` crate (version 1.x) as a dependency of the feature,
  using `arb = ["arbitrary"]`.

[`Arbitrary`]: https://docs.rs/arbitrary/1.0.0/arbitrary/trait.Arbitrary.html

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
