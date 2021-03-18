use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

/// Generate Rust structs from a `dbc` file.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Path to a `.dbc` file
    dbc_path: PathBuf,
    /// Target directory to write Rust source file(s) to
    out_path: PathBuf,
    /// Enable debug printing
    #[structopt(long)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    dbc_codegen::codegen(args.dbc_path, args.out_path, args.debug)?;
    Ok(())
}
