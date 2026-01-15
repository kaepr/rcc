mod args;
mod driver;

use crate::args::Args;
use crate::driver::{assemble, compile, preprocess};
use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    preprocess(&args.file_path);
    compile(&args);
    assemble(&args.file_path);

    Ok(())
}
