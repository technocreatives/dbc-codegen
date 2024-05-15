# CAN DBC code generator for Rust

Generates Rust messages from a `dbc` file.

⚠️ This is experimental - use with caution. ⚠️

## Installation

Install published version using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install dbc-codegen-cli
```
Install latest version from the git repository:

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

    let config = Config::builder()
        .dbc_name("example.dbc")
        .dbc_content(&dbc_file)
        //.allow_dead_code(true) // Don't emit warnings if not all generated code is used
        //.impl_arbitrary(FeatureConfig::Gated("arbitrary")) // Optional impls.
        //.impl_debug(FeatureConfig::Always)                 // See rustdoc for more,
        //.check_ranges(FeatureConfig::Never)                // or look below for an example.
        .build();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("messages.rs");
    let mut out = std::io::BufWriter::new(std::fs::File::create(&dest_path).unwrap());
    dbc_codegen::codegen(config, &mut out).expect("dbc-codegen failed");
}
```

and including the following snippet in your `main.rs` or `lib.rs`:

```rust
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}
```

## Using generated Rust code

dbc-codegen generates a Rust file that is expected to be in a cargo project.
Here is an example [`testing/can-messages/Cargo.toml`](testing/can-messages/Cargo.toml) which defines dependencies and features that are used in generated message file.

### Project setup

To use the code, add `mod messages` to your `lib.rs` (or `main.rs`).
You will most likely want to interact with the generated `Messages` enum, and call `Messages::from_can_message(id, &payload)`.

Note: The generated code contains a lot of documentation.
Give it a try:
```bash
cargo doc --open
```

### Optional impls

The generator config has the following flags that control what code gets generated:

- `impl_debug`: enables `#[derive(Debug)]` for messages.
- `impl_arbitrary`: enables implementation of [`Arbitrary`] trait.
  Also requires you to add `arbitrary` crate (version 1.x) as a dependency of the crate.
  [`Arbitrary`]: https://docs.rs/arbitrary/1.0.0/arbitrary/trait.Arbitrary.html
- `impl_error`: Implements `std::error::Error` for `CanError`. This makes it easy to use crates like `anyhow` for error handling.
- `check_ranges`: adds range checks in signal setters. (Enabled by default)

These implementations can be enabled, disabled, or placed behind feature guards, like so:

```rust
Config::builder()
    // this will generate Debug implementations
    .impl_debug(FeatureConfig::Always)

    // this will generate Error implementations behind `#[cfg(feature = "std")]` guards
    .impl_error(FeatureConfig::Gated("std"))

    // this will disable range checks
    .check_ranges(FeatureConfig::Never)
```

### no_std

The generated code is no_std compatible, unless you enable `impl_error`.

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
    _Other(bool),
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
        x => BarType::_Other(x),
    }
}
```

## Development

### lorri for Nix

If using Nix, dbc-codegen is integrated with [lorri](https://github.com/nix-community/lorri) for easy project dependency management. To enable, create a symlink in the top-level working directory:

```sh
ln -s envrc.lorri .envrc
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
