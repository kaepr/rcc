use crate::parser;
use thiserror::Error;

#[derive(Default, Clone, PartialEq, Debug, Error)]
pub enum CodegenError {
    #[default]
    #[error("unknown node")]
    UnknownNode,
}

pub type Identifer<'src> = &'src str;

#[derive(Debug, PartialEq)]
pub struct Program<'src> {
    function_def: FunctionDef<'src>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDef<'src> {
    name: Identifer<'src>,
    instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm(usize),
    Register,
}

fn program<'src>(program: parser::Program<'src>) -> Program<'src> {
    Program {
        function_def: function_definition(program.function_def),
    }
}

fn function_definition<'src>(function_def: parser::FunctionDef<'src>) -> FunctionDef<'src> {
    FunctionDef {
        name: function_def.name,
        instructions: instructions(function_def.body),
    }
}

fn instructions(statement: parser::Statement) -> Vec<Instruction> {
    match statement {
        parser::Statement::Return(expr) => {
            vec![
                Instruction::Mov(expression(expr), Operand::Register),
                Instruction::Ret,
            ]
        }
    }
}

fn expression(expr: parser::Expr) -> Operand {
    match expr {
        parser::Expr::Constant(c) => Operand::Imm(c),
    }
}

pub fn codegen<'src>(ast: parser::Program<'src>) -> Result<Program<'src>, CodegenError> {
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
        let assembly_ast = codegen(ast).unwrap();

        assert_eq!(
            assembly_ast,
            Program {
                function_def: FunctionDef {
                    name: "main",
                    instructions: vec![
                        Instruction::Mov(Operand::Imm(0), Operand::Register),
                        Instruction::Ret,
                    ],
                }
            }
        )
    }
}
