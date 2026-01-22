use crate::token::Token;
use thiserror::Error;

// inspired from Implementing a Lox interpreter in Rust
// https://www.youtube.com/watch?v=mNOLaw-_Buc

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("something went wrong")]
    Invalid,
}

struct Lexer<'a> {
    source: &'a str,
    start: usize,
    current: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
        }
    }

    fn token(&mut self, c: char) -> Result<Token<'_>, LexerError> {
        let token = match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ';' => Token::Semicolon,
            _ => todo!(),
        };
        Ok(token)
    }

    pub fn ident(&mut self) -> Token<'a> {
        todo!()
    }

    fn tokenize(&mut self) -> Result<Vec<Token<'a>>, LexerError> {
        let mut tokens = Vec::new();

        while self.current < self.source.len() {
            let c = &self.source[self.current..].chars().next().unwrap();

            if Lexer::is_whitespace(c) {
                self.current += c.len_utf8();
                continue;
            }

            let token = self.token(*c)?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }
}

pub fn lex<'a>(input: &'a str) -> Result<Vec<Token<'a>>, LexerError> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() -> Result<(), LexerError> {
        assert_eq!(lex("")?, vec![]);
        Ok(())
    }

    #[test]
    fn singles() -> Result<(), LexerError> {
        assert_eq!(lex("(")?, vec![Token::OpenParen]);
        assert_eq!(lex(")")?, vec![Token::CloseParen]);
        assert_eq!(lex("{")?, vec![Token::OpenBrace]);
        assert_eq!(lex("}")?, vec![Token::CloseBrace]);
        assert_eq!(lex(";")?, vec![Token::Semicolon]);

        Ok(())
    }

    // #[test]
    // fn keywords() {
    //     assert_eq!(
    //         lex("int"),
    //         vec![Token::keyword(crate::token::KeywordKind::Int)]
    //     );

    //     assert_eq!(
    //         lex("return"),
    //         vec![Token::keyword(crate::token::KeywordKind::Return)]
    //     );

    //     assert_eq!(
    //         lex("void"),
    //         vec![Token::keyword(crate::token::KeywordKind::Void)]
    //     )
    // }
}
