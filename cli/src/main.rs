mod args;
mod driver;

use crate::args::Args;
use crate::driver::{assemble, compile, preprocess};
use clap::Parser;
use env_logger;

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .init();

    log::info!("Starting compilation ...");
    let args = Args::parse();

    preprocess(&args.file_path);
    compile(&args);

    if args.lex || args.codegen || args.parse {
        return Ok(());
    }

    assemble(&args.file_path);
    log::info!("Success !");

    Ok(())
}
