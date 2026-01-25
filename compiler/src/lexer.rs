use logos::{Lexer, Logos};
use thiserror::Error;

#[derive(Default, Clone, PartialEq, Debug, Error)]
pub enum LexError {
    #[default]
    #[error("unknown character or sequence")]
    UnknownToken,
    #[error("identifier cannot start with a digit")]
    InvalidIdentifier,
}

fn invalid_identifier<'a>(_: &mut Lexer<'a, Token<'a>>) -> Result<Token<'a>, LexError> {
    Err(LexError::InvalidIdentifier)
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(skip r"//.*")] // single line comments
#[logos(skip r"/\*([^*]|\*[^/])*\*/")] // multi line comments
#[logos(error = LexError)]
pub enum Token<'a> {
    #[token("int")]
    Int,
    #[token("void")]
    Void,
    #[token("return")]
    Return,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token(";")]
    Semicolon,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),
    #[regex(r"[0-9]+", |lex| lex.slice())]
    Constant(&'a str),
    #[regex(r"[0-9]+[a-zA-Z_][a-zA-Z0-9_]*", invalid_identifier)]
    InvalidIdentifier,
}

pub fn lex<'a>(source: &'a str) -> Vec<Result<Token<'a>, LexError>> {
    let lexer = Token::lexer(source);
    lexer.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown() {
        assert_eq!(lex("λ"), vec![Err(LexError::UnknownToken)])
    }

    #[test]
    fn empty() {
        assert_eq!(lex(""), vec![]);
    }

    #[test]
    fn singles() {
        assert_eq!(lex("("), vec![Ok(Token::OpenParen)]);
        assert_eq!(lex(")"), vec![Ok(Token::CloseParen)]);
        assert_eq!(lex("{"), vec![Ok(Token::OpenBrace)]);
        assert_eq!(lex("}"), vec![Ok(Token::CloseBrace)]);
        assert_eq!(lex(";"), vec![Ok(Token::Semicolon)]);
    }

    #[test]
    fn program() {
        assert_eq!(
            lex(r"int main(void)
            { return
            2; } "),
            vec![
                Ok(Token::Int),
                Ok(Token::Identifier("main")),
                Ok(Token::OpenParen),
                Ok(Token::Void),
                Ok(Token::CloseParen),
                Ok(Token::OpenBrace),
                Ok(Token::Return),
                Ok(Token::Constant("2")),
                Ok(Token::Semicolon),
                Ok(Token::CloseBrace),
            ]
        );
    }

    #[test]
    fn invalid_identifer() {
        assert_eq!(lex("1foo"), vec![Err(LexError::InvalidIdentifier)]);
        assert_eq!(lex("123foo"), vec![Err(LexError::InvalidIdentifier)]);
    }
}
