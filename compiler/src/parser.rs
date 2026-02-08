use std::borrow::Cow;
use std::iter::Peekable;
use std::marker::PhantomData;

use crate::lexer::Token;
use thiserror::Error;

const MIN_PREC: usize = 0;

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
    #[error("malformed factor")]
    MalformedFactor,
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
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Constant(isize),
    Unary(UnaryOperator, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
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
        let return_val = self.parse_expr(MIN_PREC)?;
        self.expect(Token::Semicolon)?;
        Ok(Statement::Return(return_val))
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
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
                let inner_expr = self.parse_factor()?;
                Ok(Expr::Unary(operator, Box::new(inner_expr)))
            }
            Token::OpenParen => {
                self.take_one();
                let inner_expr = self.parse_expr(MIN_PREC)?;
                self.expect(Token::CloseParen)?;
                Ok(inner_expr)
            }
            _ => Err(ParseError::MalformedFactor),
        }
    }

    fn parse_expr(&mut self, min_prec: usize) -> Result<Expr, ParseError> {
        let mut left_expr = self.parse_factor()?;
        let mut token = self.tokens.peek().ok_or(ParseError::UnexpectedEof)?.clone();

        while token.is_binary_operator() && token.precedence() >= min_prec {
            let operator: BinaryOperator = token.clone().into();
            let _ = self.take_one();
            let right_expr = self.parse_expr(token.precedence() + 1)?;
            left_expr = Expr::Binary(operator, Box::new(left_expr), Box::new(right_expr));
            match self.tokens.peek() {
                Some(t) => {
                    token = t.clone();
                }
                None => {
                    break;
                }
            }
        }

        Ok(left_expr)
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

impl From<&Token<'_>> for UnaryOperator {
    fn from(value: &Token<'_>) -> Self {
        match value {
            Token::BitwiseComplement => UnaryOperator::Complement,
            Token::Negation => UnaryOperator::Negate,
            _ => unreachable!(),
        }
    }
}

impl From<Token<'_>> for BinaryOperator {
    fn from(value: Token<'_>) -> Self {
        match value {
            Token::Asterisk => BinaryOperator::Multiply,
            Token::Percent => BinaryOperator::Remainder,
            Token::Plus => BinaryOperator::Add,
            Token::Negation => BinaryOperator::Subtract,
            Token::ForwardSlash => BinaryOperator::Divide,
            _ => unreachable!(),
        }
    }
}

impl From<&Token<'_>> for BinaryOperator {
    fn from(value: &Token<'_>) -> Self {
        match value {
            Token::Asterisk => BinaryOperator::Multiply,
            Token::Percent => BinaryOperator::Remainder,
            Token::Plus => BinaryOperator::Add,
            Token::Negation => BinaryOperator::Subtract,
            Token::ForwardSlash => BinaryOperator::Divide,
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

    #[test]
    fn basic_add() {
        let source = r"
            int main(void) {
                return 1 + 2;
            }
        ";

        let tokens = lex(source).into_iter().flatten().collect();
        let ast = parse(tokens).unwrap();

        assert_eq!(
            ast,
            Program {
                function_def: FunctionDef {
                    name: "main".into(),
                    body: Statement::Return(Expr::Binary(
                        BinaryOperator::Add,
                        Box::new(Expr::Constant(1)),
                        Box::new(Expr::Constant(2))
                    ))
                }
            }
        );
    }

    #[test]
    fn complex_expr() {
        let source = r"
            int main(void) {
                return 1 * 2 - 3 * (4 + 5);
            }
        ";

        let tokens = lex(source).into_iter().flatten().collect();
        let ast = parse(tokens).unwrap();

        assert_eq!(
            ast,
            Program {
                function_def: FunctionDef {
                    name: "main".into(),
                    body: Statement::Return(Expr::Binary(
                        BinaryOperator::Subtract,
                        Box::new(Expr::Binary(
                            BinaryOperator::Multiply,
                            Box::new(Expr::Constant(1)),
                            Box::new(Expr::Constant(2))
                        )),
                        Box::new(Expr::Binary(
                            BinaryOperator::Multiply,
                            Box::new(Expr::Constant(3)),
                            Box::new(Expr::Binary(
                                BinaryOperator::Add,
                                Box::new(Expr::Constant(4)),
                                Box::new(Expr::Constant(5))
                            ))
                        ))
                    )),
                }
            }
        );
    }
}
