use std::borrow::Cow;
use thiserror::Error;

use crate::parser::{self, Statement};
use crate::util::make_temporary;

#[derive(Default, Clone, PartialEq, Debug, Error)]
pub enum TackyError {
    #[default]
    #[error("unknown node")]
    UnknownNode,
}

pub type Identifer<'src> = Cow<'src, str>;

#[derive(Debug, PartialEq)]
pub struct Program<'src> {
    pub function_def: FunctionDef<'src>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDef<'src> {
    pub name: Identifer<'src>,
    pub instructions: Vec<Instruction<'src>>,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug, PartialEq)]
pub enum Instruction<'src> {
    Unary(UnaryOperator, Val<'src>, Val<'src>),
    Return(Val<'src>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Val<'src> {
    Constant(isize),
    Var(Identifer<'src>),
}

fn convert_unop(unop: &parser::UnaryOperator) -> UnaryOperator {
    match unop {
        parser::UnaryOperator::Complement => UnaryOperator::Complement,
        parser::UnaryOperator::Negate => UnaryOperator::Negate,
    }
}

fn handle_expr<'src>(expr: &parser::Expr, instructions: &mut Vec<Instruction<'src>>) -> Val<'src> {
    match expr {
        parser::Expr::Constant(val) => Val::Constant(*val),
        parser::Expr::Unary(unary_operator, inner_expr) => {
            let src = handle_expr(inner_expr, instructions);
            let dst = Val::Var(make_temporary().into());
            let tacky_op = convert_unop(unary_operator);
            instructions.push(Instruction::Unary(tacky_op, src, dst.clone()));
            dst
        }
    }
}

fn instructions<'src>(statement: Statement) -> Vec<Instruction<'src>> {
    let mut insts = vec![];

    let ret_val = match statement {
        Statement::Return(expr) => handle_expr(&expr, &mut insts),
    };

    insts.push(Instruction::Return(ret_val));

    insts
}

fn function_definition<'src>(function_def: parser::FunctionDef<'src>) -> FunctionDef<'src> {
    FunctionDef {
        name: function_def.name,
        instructions: instructions(function_def.body),
    }
}

fn program<'src>(program: parser::Program<'src>) -> Program<'src> {
    Program {
        function_def: function_definition(program.function_def),
    }
}

pub fn tacky<'src>(ast: parser::Program<'src>) -> Result<Program<'src>, TackyError> {
    Ok(program(ast))
}
