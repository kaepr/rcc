use crate::codegen::{FunctionDef, Instruction, Operand, Program, UnaryOperator};
use std::borrow::Cow;
use thiserror::Error;

#[derive(Default, Clone, PartialEq, Debug, Error)]
pub enum EmitError {
    #[default]
    #[error("unknown error. placeholder")]
    Unknown,
}

#[cfg(target_os = "macos")]
fn fn_name<'src>(name: Cow<'src, str>) -> String {
    format!("_{}", name)
}

#[cfg(target_os = "linux")]
fn fn_name<'src>(name: Cow<'src, str>) -> String {
    name.to_string()
}

fn platform_suffix() -> String {
    if cfg!(target_os = "macos") {
        return "".to_string();
    }

    ".section .note.GNU-stack,\"\",@progbits".to_string()
}

fn emit_operand<'src>(operand: Operand<'src>) -> String {
    match operand {
        Operand::Imm(v) => format!("${}", v),
        Operand::Reg(register) => match register {
            crate::codegen::Register::AX => format!("%eax"),
            crate::codegen::Register::R10 => format!("%r10d"),
        },
        Operand::Stack(stack_loc) => format!("{stack_loc}(%rbp)"),
        _ => unreachable!(),
    }
}

fn emit_unary_operator(op: UnaryOperator) -> String {
    match op {
        UnaryOperator::Neg => format!("negl"),
        UnaryOperator::Not => format!("notl"),
    }
}

fn emit_instruction(instruction: Instruction) -> String {
    match instruction {
        Instruction::Ret => "movq    %rbp, %rsp\n    popq    %rbp\n    ret".to_string(),
        Instruction::Mov { src, dst } => {
            format!("movl    {}, {}", emit_operand(src), emit_operand(dst))
        }
        Instruction::Unary { operator, operand } => format!(
            "{}    {}",
            emit_unary_operator(operator),
            emit_operand(operand)
        ),
        Instruction::AllocateStack(stack_loc) => format!("subq    ${}, %rsp", stack_loc.abs()),
    }
}

fn emit_instructions(instructions: Vec<Instruction>, output: &mut String) {
    instructions.into_iter().for_each(|i| {
        output.push_str(&format!("    {}\n", emit_instruction(i)));
    });
}

fn emit_function<'src>(function_def: FunctionDef<'src>, output: &mut String) {
    output.push_str(&format!(
        "    .globl {}\n",
        fn_name(function_def.name.clone()),
    ));
    output.push_str(&format!("{}:\n", fn_name(function_def.name)));
    output.push_str(&format!("    pushq   %rbp\n    movq    %rsp, %rbp\n",));
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
