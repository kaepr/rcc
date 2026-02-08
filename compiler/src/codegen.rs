use crate::tacky::{self};
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

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mult,
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
    Binary {
        operator: BinaryOperator,
        operand1: Operand<'src>,
        operand2: Operand<'src>,
    },
    Idiv(Operand<'src>),
    Cdq,
    AllocateStack(isize),
    Ret,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    AX,
    DX,
    R10,
    R11,
}

#[derive(Debug, PartialEq, Clone)]
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

impl From<tacky::BinaryOperator> for BinaryOperator {
    fn from(value: tacky::BinaryOperator) -> Self {
        type BinOp = tacky::BinaryOperator;
        match value {
            BinOp::Add => BinaryOperator::Add,
            BinOp::Multiply => BinaryOperator::Mult,
            BinOp::Subtract => BinaryOperator::Sub,
            _ => unreachable!(),
        }
    }
}

impl<'src> From<tacky::Val<'src>> for Operand<'src> {
    fn from(value: tacky::Val<'src>) -> Self {
        match value {
            tacky::Val::Constant(v) => Operand::Imm(v),
            tacky::Val::Var(ident) => Operand::Pseudo(ident),
        }
    }
}

fn handle_instruction(instruction: tacky::Instruction) -> Vec<Instruction> {
    match instruction {
        tacky::Instruction::Unary(unary_operator, src, dst) => vec![
            Instruction::Mov {
                src: src.into(),
                dst: dst.clone().into(),
            },
            Instruction::Unary {
                operator: unary_operator.into(),
                operand: dst.into(),
            },
        ],
        tacky::Instruction::Binary {
            op,
            src1,
            src2,
            dst,
        } => {
            type BinOp = tacky::BinaryOperator;
            match op {
                BinOp::Divide => vec![
                    Instruction::Mov {
                        src: src1.into(),
                        dst: Operand::Reg(Register::AX),
                    },
                    Instruction::Cdq,
                    Instruction::Idiv(src2.into()),
                    Instruction::Mov {
                        src: Operand::Reg(Register::AX),
                        dst: dst.into(),
                    },
                ],
                BinOp::Remainder => vec![
                    Instruction::Mov {
                        src: src1.into(),
                        dst: Operand::Reg(Register::AX),
                    },
                    Instruction::Cdq,
                    Instruction::Idiv(src2.into()),
                    Instruction::Mov {
                        src: Operand::Reg(Register::DX),
                        dst: dst.into(),
                    },
                ],
                _ => vec![
                    Instruction::Mov {
                        src: src1.into(),
                        dst: dst.clone().into(),
                    },
                    Instruction::Binary {
                        operator: op.into(),
                        operand1: src2.into(),
                        operand2: dst.into(),
                    },
                ],
            }
        }
        tacky::Instruction::Return(val) => vec![
            Instruction::Mov {
                src: val.into(),
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
            Instruction::Binary {
                operator,
                operand1,
                operand2,
            } => Instruction::Binary {
                operator: operator,
                operand1: handle_pseudo_operand(operand1, &mut hash_map, &mut stack_loc),
                operand2: handle_pseudo_operand(operand2, &mut hash_map, &mut stack_loc),
            },
            Instruction::Idiv(o) => {
                Instruction::Idiv(handle_pseudo_operand(o, &mut hash_map, &mut stack_loc))
            }
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
        Instruction::Idiv(Operand::Imm(v)) => vec![
            Instruction::Mov {
                src: Operand::Imm(v),
                dst: Operand::Reg(Register::R10),
            },
            Instruction::Idiv(Operand::Reg(Register::R10)),
        ],
        Instruction::Binary {
            operator: op @ BinaryOperator::Add | op @ BinaryOperator::Sub,
            operand1: op1 @ Operand::Stack(_),
            operand2: op2 @ Operand::Stack(_),
        } => vec![
            Instruction::Mov {
                src: op1,
                dst: Operand::Reg(Register::R10),
            },
            Instruction::Binary {
                operator: op,
                operand1: Operand::Reg(Register::R10),
                operand2: op2,
            },
        ],
        Instruction::Binary {
            operator: BinaryOperator::Mult,
            operand1,
            operand2: dst @ Operand::Stack(_),
        } => vec![
            Instruction::Mov {
                src: dst.clone(),
                dst: Operand::Reg(Register::R11),
            },
            Instruction::Binary {
                operator: BinaryOperator::Mult,
                operand1: operand1,
                operand2: Operand::Reg(Register::R11),
            },
            Instruction::Mov {
                src: Operand::Reg(Register::R11),
                dst: dst,
            },
        ],
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
    use crate::tacky::tacky;

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
