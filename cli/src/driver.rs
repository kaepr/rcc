use crate::args::Args;
use compiler::{FINAL_STAGE, Stage, StageOutput, compile as rcc_compiler};
use log::debug;
use std::error::Error;
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

fn stage(args: &Args) -> Stage {
    if args.lex {
        return Stage::Lex;
    }

    if args.parse {
        return Stage::Parse;
    }

    if args.codegen {
        return Stage::Codegen;
    }

    FINAL_STAGE
}

fn process_errors<T, E>(input: Vec<Result<T, E>>)
where
    E: Error,
{
    let errors: Vec<_> = input.iter().filter_map(|r| r.as_ref().err()).collect();
    if !errors.is_empty() {
        errors.iter().for_each(|e| log::debug!("{:?}", e));
        std::process::exit(1);
    }
}

fn process_output(output: StageOutput) {
    match output {
        StageOutput::Lex(tokens) => {
            tokens
                .iter()
                .filter(|t| t.is_ok())
                .for_each(|t| log::debug!("{:?}", t));

            process_errors(tokens);
        }
    }
}

pub fn compile(args: &Args) {
    let mut file_path = args.file_path.clone();
    file_path.set_extension("i");
    let source = fs::read_to_string(file_path).unwrap();

    log::debug!("file contents: {:?}", source);
    let _ = cleanup(&args.file_path);

    let stage = stage(args);
    let output = rcc_compiler(&source, stage);
    process_output(output);
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
