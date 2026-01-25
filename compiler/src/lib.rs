use lexer::{LexerError, Token, lex};

pub mod lexer;
pub mod parser;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Stage {
    Lex,
    Parse,
    Codegen,
}

pub const FINAL_STAGE: Stage = Stage::Codegen;

pub enum StageOutput<'a> {
    Lex(Vec<Result<Token<'a>, LexerError>>),
}

pub fn compile<'a>(source: &'a str, stage: Stage) -> StageOutput<'a> {
    match stage {
        Stage::Lex => StageOutput::Lex(lex(source)),
        Stage::Parse => todo!(),
        Stage::Codegen => todo!(),
    }
}
