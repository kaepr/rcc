use std::borrow::Cow;
use std::iter::Peekable;
use std::marker::PhantomData;

use crate::lexer::Token;
use thiserror::Error;

#[derive(Default, Clone, PartialEq, Debug, Error)]
pub enum ParseError {
    #[default]
    #[error("unknown token found")]
    UnknownToken,
    #[error("expected: {expected}, found: {found}")]
    UnexpectedToken { expected: String, found: String },
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error("invalid integer: {int}")]
    InvalidInteger { int: String },
    #[error("expected eof, found: {found}")]
    ExpectedEof { found: String },
    #[error("malformed expression found: {found}")]
    MalformedExpression { found: String },
}

pub type Identifer<'src> = Cow<'src, str>;

#[derive(Debug, PartialEq)]
pub struct Program<'src> {
    pub function_def: FunctionDef<'src>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDef<'src> {
    pub name: Identifer<'src>,
    pub body: Statement,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expr),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Constant(isize),
    Unary(UnaryOperator, Box<Expr>),
}

struct Parser<'src, I>
where
    I: Iterator<Item = Token<'src>>,
{
    tokens: Peekable<I>,
    _marker: PhantomData<&'src ()>,
}

impl<'src, I> Parser<'src, I>
where
    I: Iterator<Item = Token<'src>>,
{
    fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
            _marker: PhantomData,
        }
    }

    fn parse_program(&mut self) -> Result<Program<'src>, ParseError> {
        let function_def = self.parse_function_def()?;
        self.expect_eof()?;
        Ok(Program { function_def })
    }

    fn parse_function_def(&mut self) -> Result<FunctionDef<'src>, ParseError> {
        self.expect(Token::Int)?;
        let ident = self.expect_identifier()?;
        self.expect(Token::OpenParen)?;
        self.expect(Token::Void)?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let statement = self.parse_statement()?;
        self.expect(Token::CloseBrace)?;
        Ok(FunctionDef {
            name: ident,
            body: statement,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect(Token::Return)?;
        let return_val = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(Statement::Return(return_val))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let token = self.tokens.peek().ok_or(ParseError::UnexpectedEof)?.clone();

        match token {
            Token::Constant(val) => {
                self.take_one();

                match val.parse::<usize>() {
                    Ok(val) => Ok(Expr::Constant(
                        val.try_into().expect("should be representable in isize"),
                    )),
                    Err(_e) => Err(ParseError::InvalidInteger {
                        int: val.to_string(),
                    }),
                }
            }
            Token::BitwiseComplement | Token::Negation => {
                self.take_one();
                let operator: UnaryOperator = token.into();
                let expr = self.parse_expr()?;
                Ok(Expr::Unary(operator, Box::new(expr)))
            }
            Token::OpenParen => {
                self.take_one();
                let expr = self.parse_expr()?;
                self.expect(Token::CloseParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::MalformedExpression {
                found: format!("{:?}", token),
            }),
        }
    }

    fn expect(&mut self, expected: Token<'src>) -> Result<(), ParseError> {
        let next = self.tokens.next().ok_or(ParseError::UnexpectedEof)?;

        if next == expected {
            return Ok(());
        }

        Err(ParseError::UnexpectedToken {
            expected: format!("{:?}", expected),
            found: format!("{:?}", next),
        })
    }

    fn take_one(&mut self) -> Token<'src> {
        self.tokens.next().unwrap()
    }

    fn expect_identifier(&mut self) -> Result<Identifer<'src>, ParseError> {
        match self.tokens.next() {
            Some(Token::Identifier(ident)) => Ok(ident.into()),
            Some(t) => Err(ParseError::UnexpectedToken {
                expected: "Token::Identifier(ident)".into(),
                found: format!("{:?}", t),
            }),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn expect_eof(&mut self) -> Result<(), ParseError> {
        match self.tokens.next() {
            Some(t) => Err(ParseError::ExpectedEof {
                found: format!("{:?}", t),
            }),
            None => Ok(()),
        }
    }
}

impl From<Token<'_>> for UnaryOperator {
    fn from(value: Token<'_>) -> Self {
        match value {
            Token::BitwiseComplement => UnaryOperator::Complement,
            Token::Negation => UnaryOperator::Negate,
            _ => unreachable!(),
        }
    }
}

pub fn parse<'src>(tokens: Vec<Token<'src>>) -> Result<Program<'src>, ParseError> {
    let mut parser = Parser::new(tokens.into_iter());
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn program() {
        let source = r"
            int main(void) {
                return 0;
            }
        ";

        let tokens = lex(source).into_iter().flatten().collect();
        let ast = parse(tokens).unwrap();

        assert_eq!(
            ast,
            Program {
                function_def: FunctionDef {
                    name: "main".into(),
                    body: Statement::Return(Expr::Constant(0))
                }
            }
        )
    }
}
