use anyhow::Result;
use dbc_codegen::{Config, FeatureConfig};
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

    let config = Config::builder()
        .dbc_name("example.dbc")
        .dbc_content(&dbc_file)
        .debug_prints(true)
        .impl_debug(FeatureConfig::Always)
        .impl_error(FeatureConfig::Gated("std"))
        .impl_arbitrary(FeatureConfig::Gated("arb"))
        .check_ranges(FeatureConfig::Always)
        .build();

    dbc_codegen::codegen(config, &mut out)?;

    out.flush()?;

    Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(out_file)
        .output()
        .expect("failed to execute rustfmt");

    Ok(())
}
