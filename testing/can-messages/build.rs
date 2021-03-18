use anyhow::Result;
use std::{
    fs::{self, File},
    io::BufWriter,
};

fn main() -> Result<()> {
    let dbc_file = fs::read("../dbc-examples/example.dbc")?;
    let mut out = BufWriter::new(File::create("src/messages.rs")?);
    println!("cargo:rerun-if-changed=../dbc-examples/example.dbc");

    dbc_codegen::codegen("example.dbc", &dbc_file, &mut out, true)?;
    Ok(())
}
