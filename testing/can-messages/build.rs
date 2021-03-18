use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let dbc_file: PathBuf = PathBuf::from("../dbc-examples/example.dbc");
    let out: PathBuf = PathBuf::from("src/");
    println!("cargo:rerun-if-changed=../dbc-examples/example.dbc");
    dbc_codegen::codegen(dbc_file, out, true)?;
    Ok(())
}
