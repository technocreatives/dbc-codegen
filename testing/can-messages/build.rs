use anyhow::Result;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    process::Command,
};

fn main() -> Result<()> {
    let out_file = "src/messages.rs";
    let dbc_file = fs::read("../dbc-examples/example.dbc")?;
    let mut out = BufWriter::new(File::create(out_file)?);
    println!("cargo:rerun-if-changed=../dbc-examples/example.dbc");
    println!("cargo:rerun-if-changed=../../src");

    dbc_codegen::codegen("example.dbc", &dbc_file, &mut out, true)?;

    out.flush()?;

    if cfg!(target_os = "linux") {
        Command::new("rustfmt")
            .arg("--edition")
            .arg("2018")
            .arg(out_file)
            .output()
            .expect("failed to execute rustfmt");
    }

    Ok(())
}
