use crate::args::Args;
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
        println!("Successfully preprocessed to: {:?}", output);
    } else {
        eprintln!("Failed to preprocess file.");
        let _ = cleanup(input);
        std::process::exit(1);
    }
}

pub fn compile(_args: &Args) {
    println!("TODO: compiler implementation here. Write to a .s file");
    println!("Exit early as specified in the args.");
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

    if status.success() {
        println!("Successfully generated executable.");
    } else {
        eprintln!("Failed to generate executable.");
        let _ = cleanup(input);
        std::process::exit(1);
    }
}

fn cleanup(path: &PathBuf) -> std::io::Result<()> {
    let mut path = path.clone();
    path.set_extension("i");
    if path.exists() {
        fs::remove_file(&path)?;
        println!(".i file cleaned up !")
    } else {
        println!("No .i file to cleanup !")
    }

    path.set_extension("s");
    if path.exists() {
        fs::remove_file(&path)?;
        println!(".s file cleaned up !")
    } else {
        println!("No .s file to cleanup !")
    }

    Ok(())
}
