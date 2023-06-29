#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),

    Integer(i32),

    Plus,
    Minus,
    Multiply,
    Divide,
    Bang,
    Lt,
    Gt,
    Eq,
    NotEq,

    Assign,

    Illegal(String),
    Eof,

    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
