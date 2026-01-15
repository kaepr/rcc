use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// Input C file.
    pub file_path: PathBuf,

    /// Runs lexer. Will not emit any files.
    #[arg(long)]
    pub lex: bool,

    /// Runs parser. Will not emit any files.
    #[arg(long)]
    pub parse: bool,

    /// Runs codegen. Will not emit any files.
    #[arg(long)]
    pub codegen: bool,
}
