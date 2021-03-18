# Development

Thanks for looking into contributing to this project!

## Quick overview

```bash
# generate messages.rs to look at results
cargo run -p dbc-codegen-cli -- testing/dbc-examples/example.dbc testing/can-messages/src

# build everything
cargo build --all

# run all tests
cargo test --all

# format before committing anything
cargo fmt --all
```

## Testing setup

Please refer to the [testing README file](./testing/README.adoc)

## Generate .kdc from .dbc

Use [canmatrix](https://github.com/ebroecker/canmatrix) if you need to generate a new `.kcd` file from a `.dbc`.

```bash
# https://canmatrix.readthedocs.io/en/latest/installation.html
pip install canmatrix
pip install git+https://github.com/ebroecker/canmatrix#egg=canmatrix[kcd]

# generate .kcd
canconvert testing/dbc-examples/example.dbc testing/dbc-examples/example.kcd
```
