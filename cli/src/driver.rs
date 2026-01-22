use crate::args::Args;
use compiler::lexer::{self, LexerError};
use log::debug;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn preprocess(input: &PathBuf) {
    let mut output = input.clone();
    output.set_extension("i");

    let status = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(input)
        .arg("-o")
        .arg(&output)
        .status()
        .expect("failed to run gcc");

    if status.success() {
        debug!("Successfully preprocessed to: {:?}", output);
    } else {
        debug!("Failed to preprocess file.");
        let _ = cleanup(input);
        std::process::exit(1);
    }
}

pub fn compile(args: &Args) {
    let mut file_path = args.file_path.clone();
    file_path.set_extension("i");
    let source = fs::read_to_string(file_path).unwrap();

    log::debug!("file contents: {:?}", source);
    let _ = cleanup(&args.file_path);

    let tokens = lexer::lex(&source);
    let errors: Vec<&LexerError> = tokens.iter().filter_map(|t| t.as_ref().err()).collect();

    for token in &tokens {
        log::debug!("{:?}", token);
    }

    if !errors.is_empty() {
        for error in errors {
            log::debug!("{:?}", error);
        }
        std::process::exit(1);
    }

    if args.lex {
        return;
    }

    if args.parse {
        return;
    }

    if args.codegen {
        return;
    }

    // write to .s file
}

pub fn assemble(input: &PathBuf) {
    let mut assembly = input.clone();
    assembly.set_extension("s");
    let mut output = input.clone();
    output.set_extension("");

    let status = Command::new("gcc")
        .arg(&assembly)
        .arg("-o")
        .arg(&output)
        .status()
        .expect("failed to run gcc");

    let _ = cleanup(input);
    if status.success() {
        log::debug!("Successfully generated executable.");
    } else {
        log::debug!("Failed to generate executable.");
        std::process::exit(1);
    }
}

fn cleanup(path: &PathBuf) -> std::io::Result<()> {
    let mut path = path.clone();
    path.set_extension("i");
    if path.exists() {
        fs::remove_file(&path)?;
        log::debug!(".i file cleaned up !")
    } else {
        log::debug!("No .i file to cleanup !")
    }

    path.set_extension("s");
    if path.exists() {
        fs::remove_file(&path)?;
        log::debug!(".s file cleaned up !")
    } else {
        log::debug!("No .s file to cleanup !")
    }

    Ok(())
}
