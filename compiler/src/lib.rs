use codegen::{CodegenError, codegen};
use lexer::{LexError, Token, lex};
use parser::{ParseError, Program, parse};

pub mod codegen;
pub mod lexer;
pub mod parser;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Stage {
    Lex,
    Parse,
    Codegen,
}

pub const FINAL_STAGE: Stage = Stage::Codegen;

pub enum StageOutput<'src> {
    Lex(Vec<Result<Token<'src>, LexError>>),
    Parse(Result<Program<'src>, ParseError>),
    Codegen(Result<codegen::Program<'src>, CodegenError>),
}

pub fn compile<'a>(source: &'a str, stage: Stage) -> StageOutput<'a> {
    match stage {
        Stage::Lex => StageOutput::Lex(lex(source)),
        Stage::Parse => {
            let tokens = lex(source)
                .into_iter()
                .map(|t| t.expect("found error while lexing"))
                .collect();

            StageOutput::Parse(parse(tokens))
        }
        Stage::Codegen => {
            let tokens = lex(source)
                .into_iter()
                .map(|t| t.expect("found error while lexing"))
                .collect();
            let ast = parse(tokens).expect("found error while parsing");

            StageOutput::Codegen(codegen(ast))
        }
    }
}
