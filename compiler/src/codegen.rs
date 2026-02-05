use crate::tacky;
use std::{borrow::Cow, collections::HashMap, vec};
use thiserror::Error;

#[derive(Default, Clone, PartialEq, Debug, Error)]
pub enum CodegenError {
    #[default]
    #[error("unknown node")]
    UnknownNode,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Neg,
    Not,
}

pub type Identifier<'src> = Cow<'src, str>;

#[derive(Debug, PartialEq)]
pub struct Program<'src> {
    pub function_def: FunctionDef<'src>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDef<'src> {
    pub name: Identifier<'src>,
    pub instructions: Vec<Instruction<'src>>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction<'src> {
    Mov {
        src: Operand<'src>,
        dst: Operand<'src>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Operand<'src>,
    },
    AllocateStack(isize),
    Ret,
}

#[derive(Debug, PartialEq)]
pub enum Register {
    AX,
    R10,
}

#[derive(Debug, PartialEq)]
pub enum Operand<'src> {
    Imm(isize),
    Reg(Register),
    Pseudo(Identifier<'src>),
    Stack(isize),
}

fn program<'src>(program: tacky::Program<'src>) -> Program<'src> {
    Program {
        function_def: function_definition(program.function_def),
    }
}

fn function_definition<'src>(function_def: tacky::FunctionDef<'src>) -> FunctionDef<'src> {
    FunctionDef {
        name: function_def.name,
        instructions: handle_instructions(function_def.instructions),
    }
}

impl From<tacky::UnaryOperator> for UnaryOperator {
    fn from(value: tacky::UnaryOperator) -> Self {
        match value {
            tacky::UnaryOperator::Complement => UnaryOperator::Not,
            tacky::UnaryOperator::Negate => UnaryOperator::Neg,
        }
    }
}

fn handle_operand(op: tacky::Val) -> Operand {
    match op {
        tacky::Val::Constant(v) => Operand::Imm(v),
        tacky::Val::Var(ident) => Operand::Pseudo(ident),
    }
}

fn handle_instruction(instruction: tacky::Instruction) -> Vec<Instruction> {
    match instruction {
        tacky::Instruction::Unary(unary_operator, src, dst) => vec![
            Instruction::Mov {
                src: handle_operand(src),
                dst: handle_operand(dst.clone()),
            },
            Instruction::Unary {
                operator: unary_operator.into(),
                operand: handle_operand(dst),
            },
        ],
        tacky::Instruction::Return(val) => vec![
            Instruction::Mov {
                src: handle_operand(val),
                dst: Operand::Reg(Register::AX),
            },
            Instruction::Ret,
        ],
    }
}

fn handle_pseudo_operand<'src>(
    operand: Operand<'src>,
    hash_map: &mut HashMap<Identifier<'src>, isize>,
    stack_loc: &mut isize,
) -> Operand<'src> {
    match operand {
        Operand::Pseudo(ident) => match hash_map.get(&ident) {
            Some(loc) => Operand::Stack(*loc),
            None => {
                hash_map.insert(ident, *stack_loc);
                let op = Operand::Stack(*stack_loc);
                *stack_loc -= 4;
                op
            }
        },
        other => other,
    }
}

fn replace_pseudo_registers<'src>(
    instructions: Vec<Instruction<'src>>,
) -> (Vec<Instruction<'src>>, isize) {
    let mut stack_loc = -4;
    let mut hash_map: HashMap<Identifier<'src>, isize> = HashMap::new();

    let instructions = instructions
        .into_iter()
        .map(|inst| match inst {
            Instruction::Mov { src, dst } => Instruction::Mov {
                src: handle_pseudo_operand(src, &mut hash_map, &mut stack_loc),
                dst: handle_pseudo_operand(dst, &mut hash_map, &mut stack_loc),
            },
            Instruction::Unary { operator, operand } => Instruction::Unary {
                operator: operator,
                operand: handle_pseudo_operand(operand, &mut hash_map, &mut stack_loc),
            },
            other => other,
        })
        .collect();

    (instructions, stack_loc)
}

fn fixup<'src>(instruction: Instruction<'src>) -> Vec<Instruction<'src>> {
    match instruction {
        Instruction::Mov { src, dst } => match (&src, &dst) {
            (Operand::Stack(_), Operand::Stack(_)) => vec![
                Instruction::Mov {
                    src: src,
                    dst: Operand::Reg(Register::R10),
                },
                Instruction::Mov {
                    src: Operand::Reg(Register::R10),
                    dst: dst,
                },
            ],
            _ => vec![Instruction::Mov { src: src, dst: dst }],
        },
        other => vec![other],
    }
}

fn handle_instructions(instructions: Vec<tacky::Instruction>) -> Vec<Instruction> {
    let instructions = instructions
        .into_iter()
        .flat_map(|inst| handle_instruction(inst))
        .collect();

    let (mut instructions, stack_loc) = replace_pseudo_registers(instructions);
    instructions.insert(0, Instruction::AllocateStack(stack_loc + 4));

    instructions
        .into_iter()
        .flat_map(|inst| fixup(inst))
        .collect()
}

pub fn codegen<'src>(ast: tacky::Program<'src>) -> Result<Program<'src>, CodegenError> {
    Ok(program(ast))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    #[test]
    fn program() {
        let source = r"
            int main(void) {
                return 0;
            }
        ";

        let tokens = lex(source).into_iter().flatten().collect();
        let ast = parse(tokens).unwrap();
        let tacky = tacky(ast).unwrap();
        let _assembly_ast = codegen(tacky).unwrap();

        assert_eq!(2, 1 + 1);
    }
}
