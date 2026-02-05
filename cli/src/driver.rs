use crate::args::Args;
use compiler::{Stage, StageOutput, compile as rcc_compiler};
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
        log::debug!("Successfully preprocessed to: {:?}", output);
    } else {
        log::error!("Failed to preprocess file.");
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

    if args.tacky {
        return Stage::Tacky;
    }

    Stage::Emit
}

fn process_output(output: StageOutput) -> Option<String> {
    match output {
        StageOutput::Lex(tokens) => {
            let (tokens, errors) =
                tokens
                    .into_iter()
                    .fold((Vec::new(), Vec::new()), |(mut ts, mut es), token| {
                        match token {
                            Ok(t) => ts.push(t),
                            Err(e) => es.push(e),
                        }
                        (ts, es)
                    });

            tokens.iter().for_each(|t| log::debug!("{t:?}"));

            if !errors.is_empty() {
                errors.iter().for_each(|e| log::debug!("{e:?}"));
                std::process::exit(1);
            }
        }
        StageOutput::Parse(ast) => match ast {
            Ok(ast) => log::debug!("\n{ast:?}"),
            Err(e) => {
                log::debug!("{e:?}");
                std::process::exit(1);
            }
        },
        StageOutput::Tacky(ast) => match ast {
            Ok(ast) => log::debug!("\n{ast:?}"),
            Err(e) => {
                log::debug!("{e:?}");
                std::process::exit(1);
            }
        },
        StageOutput::Codegen(ast) => match ast {
            Ok(ast) => log::debug!("\n{ast:?}"),
            Err(e) => {
                log::debug!("{e:?}");
                std::process::exit(1);
            }
        },
        StageOutput::Emit(assembly) => match assembly {
            Ok(assembly) => {
                log::debug!("\n{}", assembly);
                return Some(assembly);
            }
            Err(e) => {
                log::debug!("{e:?}");
                std::process::exit(1);
            }
        },
    }

    None
}

pub fn compile(args: &Args) {
    let mut file_path = args.file_path.clone();
    file_path.set_extension("i");
    let source = fs::read_to_string(&file_path).unwrap();

    log::debug!("file contents: {:?}", source);
    let _ = cleanup(&args.file_path);

    let stage = stage(args);
    let output = rcc_compiler(&source, stage);
    let output = process_output(output);

    if let Some(output) = output {
        file_path.set_extension("s");
        match fs::write(file_path, output) {
            Ok(_) => log::info!("wrote to .s file"),
            Err(_) => log::error!("failed to write to .s file"),
        }
    }
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
        log::error!("Failed to generate executable.");
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
