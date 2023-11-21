use clap::Parser;
use dbc_codegen::Config;
use std::fs::File;
use std::{path::PathBuf, process::exit};

/// Generate Rust `struct`s from a `dbc` file.
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    /// Path to a `.dbc` file
    dbc_path: PathBuf,

    /// Target directory to write Rust source file(s) to
    out_path: PathBuf,

    /// Enable debug printing
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Cli::parse();
    let dbc_file = std::fs::read(&args.dbc_path).unwrap_or_else(|e| {
        eprintln!("could not read `{}`: {}", args.dbc_path.display(), e);
        exit(exitcode::NOINPUT);
    });
    let dbc_file_name = args
        .dbc_path
        .file_name()
        .unwrap_or_else(|| args.dbc_path.as_ref())
        .to_string_lossy();

    if !args.out_path.is_dir() {
        eprintln!(
            "Output path needs to point to a directory (checked {})",
            args.out_path.display()
        );
        exit(exitcode::CANTCREAT);
    }

    let messages_path = args.out_path.join("messages.rs");
    let mut messages_code = File::create(messages_path).unwrap_or_else(|e| {
        eprintln!(
            "Could not create `messages.rs` file in {}: {:?}",
            args.out_path.display(),
            e
        );
        exit(exitcode::CANTCREAT);
    });

    let config = Config::builder()
        .dbc_name(&dbc_file_name)
        .dbc_content(&dbc_file)
        .debug_prints(true)
        .build();

    dbc_codegen::codegen(config, &mut messages_code).unwrap_or_else(|e| {
        eprintln!("could not convert `{}`: {}", args.dbc_path.display(), e);
        if args.debug {
            eprintln!("details: {:?}", e);
        }
        exit(exitcode::NOINPUT)
    })
}
