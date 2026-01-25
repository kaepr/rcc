use crate::codegen::{FunctionDef, Instruction, Operand, Program};
use thiserror::Error;

#[derive(Default, Clone, PartialEq, Debug, Error)]
pub enum EmitError {
    #[default]
    #[error("unknown error. placeholder")]
    Unknown,
}

#[cfg(target_os = "macos")]
fn fn_name<'a>(name: &'a str) -> String {
    format!("_{}", name)
}

#[cfg(target_os = "linux")]
fn fn_name<'a>(name: &'a str) -> String {
    name.to_string()
}

fn platform_suffix() -> String {
    if cfg!(target_os = "macos") {
        return "".to_string();
    }

    ".section .note.GNU-stack,\"\",@progbits".to_string()
}

fn emit_operand(operand: Operand) -> String {
    match operand {
        Operand::Imm(v) => format!("${}", v),
        Operand::Register => "%eax".to_string(),
    }
}

fn emit_instruction(instruction: Instruction) -> String {
    match instruction {
        Instruction::Mov(src, dst) => {
            format!("movl    {}, {}", emit_operand(src), emit_operand(dst))
        }
        Instruction::Ret => "ret".to_string(),
    }
}

fn emit_instructions(instructions: Vec<Instruction>, output: &mut String) {
    instructions.into_iter().for_each(|i| {
        output.push_str(&format!("    {}\n", emit_instruction(i)));
    });
}

fn emit_function<'src>(function_def: FunctionDef<'src>, output: &mut String) {
    output.push_str(&format!("    .globl {}\n", fn_name(function_def.name)));
    output.push_str(&format!("{}:\n", fn_name(function_def.name)));
    emit_instructions(function_def.instructions, output);
}

fn emit_program<'src>(program: Program<'src>, output: &mut String) {
    emit_function(program.function_def, output);
    output.push('\n');
    output.push_str(&platform_suffix());
}

pub fn emit<'src>(program: Program<'src>) -> Result<String, EmitError> {
    let mut output: String = "".into();
    emit_program(program, &mut output);
    Ok(output)
}
