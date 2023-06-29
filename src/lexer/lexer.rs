use crate::lexer::token::Token;
use regex::Regex;

pub struct Lexer {
    input: String,
    position: Option<usize>,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: None,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        self.position = Some(self.read_position);
        self.read_position += 1;
        if self.position.unwrap() >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.position.unwrap());
        }
    }

    fn unread_char(&mut self) {
        self.read_position -= 1;
        self.position = Some(self.position.unwrap() - 1);
        self.ch = self.input.chars().nth(self.position.unwrap());
    }

    pub fn peek_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap()
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_some() && self.ch.unwrap().is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Divide,
            Some('*') => Token::Multiply,
            Some('!') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,

            Some('=') => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }

            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,

            None => Token::Eof,
            _ => {
                if self.ch.unwrap().is_ascii_alphabetic() || self.ch.unwrap() == '_' {
                    let ident = self.read_identifier();
                    match ident {
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        "true" => Token::True,
                        "false" => Token::False,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        _ => Token::Identifier(ident.to_string()),
                    }
                } else if self.ch.unwrap().is_digit(10) {
                    Token::Integer(self.read_number())
                } else {
                    Token::Illegal(self.ch.unwrap().to_string())
                }
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> &str {
        let start_position = self.position.unwrap();
        while self.ch.unwrap().is_ascii_alphabetic() || self.ch.unwrap() == '_' {
            self.read_char();
        }
        self.unread_char();

        &self.input[start_position..self.read_position]
    }

    fn read_number(&mut self) -> i32 {
        let start_position = self.position.unwrap();
        while self.ch.unwrap().is_digit(10) {
            self.read_char();
        }
        self.unread_char();
        self.input[start_position..self.read_position]
            .parse()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_1() {
        let input = "=+(){},;";
        let tests = [
            Token::Assign,
            Token::Plus,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input.to_string());

        for test in tests {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(token, test);
        }
    }

    #[test]
    fn test_next_token_2() {
        let input = "let five = 5;   ";
        let mut lexer = Lexer::new(input.to_string());

        let tests = [
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Eof,
        ];

        for test in tests {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(token, test);
        }
    }

    #[test]
    fn test_next_token_3() {
        let input = "let five = 5;
                     let ten = 10;

                     let add = fn(x, y) {
                       x + y;
                     };

                     let result = add(five, ten);";

        let mut lexer = Lexer::new(input.to_string());
        let tests = [
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Identifier("five".to_string()),
            Token::Comma,
            Token::Identifier("ten".to_string()),
            Token::RightParen,
            Token::Semicolon,
        ];

        for test in tests {
            assert_eq!(lexer.next_token(), test);
        }
    }

    #[test]
    fn test_next_token_4() {
        let input = "let five = 5;
                    let ten = 10;

                    let add = fn(x, y) {
                      x + y;
                    };

                    let result = add(five, ten);
                    !-/*5;
                    5 < 10 > 5;";

        let mut lexer = Lexer::new(input.to_string());

        let tests = [
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Identifier("five".to_string()),
            Token::Comma,
            Token::Identifier("ten".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Divide,
            Token::Multiply,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::Lt,
            Token::Integer(10),
            Token::Gt,
            Token::Integer(5),
            Token::Semicolon,
            Token::Eof,
        ];

        for test in tests {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(token, test);
        }
    }

    #[test]
    fn test_next_token_5() {
        let input = "let five = 5;
                    let ten = 10;

                    let add = fn(x, y) {
                      x + y;
                    };

                    let result = add(five, ten);
                    !-/*5;
                    5 < 10 > 5;

                    if (5 < 10) {
                        return true;
                    } else {
                        return false;
                    }

                    10 == 10;
                    10 != 9;";

        let mut lexer = Lexer::new(input.to_string());

        let tests = [
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Identifier("five".to_string()),
            Token::Comma,
            Token::Identifier("ten".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Divide,
            Token::Multiply,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::Lt,
            Token::Integer(10),
            Token::Gt,
            Token::Integer(5),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Integer(5),
            Token::Lt,
            Token::Integer(10),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RightBrace,
            Token::Integer(10),
            Token::Eq,
            Token::Integer(10),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEq,
            Token::Integer(9),
            Token::Semicolon,
            Token::Eof,
        ];

        for test in tests {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(token, test);
        }
    }
}
