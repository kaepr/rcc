#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeywordKind {
    Int,
    Return,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a str),
    Keyword(KeywordKind),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

impl<'a> Token<'a> {
    pub fn keyword(kind: KeywordKind) -> Self {
        Token::Keyword(kind)
    }

    pub fn ident(name: &'a str) -> Self {
        Token::Identifier(name)
    }
}
