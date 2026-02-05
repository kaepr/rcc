use codegen::{CodegenError, codegen};
use emit::{EmitError, emit};
use lexer::{LexError, Token, lex};
use parser::{ParseError, Program, parse};
use tacky::{TackyError, tacky};

pub mod codegen;
pub mod emit;
pub mod lexer;
pub mod parser;
pub mod tacky;
pub mod util;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Stage {
    Lex,
    Parse,
    Tacky,
    Codegen,
    Emit,
}

pub enum StageOutput<'src> {
    Lex(Vec<Result<Token<'src>, LexError>>),
    Parse(Result<Program<'src>, ParseError>),
    Tacky(Result<tacky::Program<'src>, TackyError>),
    Codegen(Result<codegen::Program<'src>, CodegenError>),
    Emit(Result<String, EmitError>),
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
        Stage::Tacky => {
            let tokens = lex(source)
                .into_iter()
                .map(|t| t.expect("found error while lexing"))
                .collect();

            let ast = parse(tokens).expect("found error while parsing");
            StageOutput::Tacky(tacky(ast))
        }
        Stage::Codegen => {
            let tokens = lex(source)
                .into_iter()
                .map(|t| t.expect("found error while lexing"))
                .collect();
            let ast = parse(tokens).expect("found error while parsing");
            let tacky = tacky(ast).expect("found error while tacky");
            StageOutput::Codegen(codegen(tacky))
        }
        Stage::Emit => {
            let tokens = lex(source)
                .into_iter()
                .map(|t| t.expect("found error while lexing"))
                .collect();
            let ast = parse(tokens).expect("found error while parsing");
            let tacky = tacky(ast).expect("found error while tacky");
            let codegen = codegen(tacky).expect("found error while codegen");

            StageOutput::Emit(emit(codegen))
        }
    }
}
