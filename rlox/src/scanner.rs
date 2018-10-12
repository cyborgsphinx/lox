use std::{iter,str};

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    // single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // one or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // literals
    Identifier, Str, Number,
    // Keywords
    And, Class, Else, False,
    Fun, For, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub contents: String,
    pub line: u32,
}

impl Token {
    fn new(kind: TokenType, contents: &str, line: u32) -> Self {
        Token { kind: kind, contents: String::from(contents), line: line }
    }
}

macro_rules! token {
    ($kind:expr, $cont:expr, $line:expr) => { Some(Token::new($kind, $cont, $line)) }
}

pub struct Scanner<'a> {
    contents: iter::Peekable<str::Chars<'a>>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { contents: source.chars().peekable(), line: 1 }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.contents.next() {
            match ch {
                '(' => return token!(TokenType::LeftParen,  "(", self.line),
                ')' => return token!(TokenType::RightParen, ")", self.line),
                '{' => return token!(TokenType::LeftBrace,  "{", self.line),
                '}' => return token!(TokenType::RightBrace, "}", self.line),
                ',' => return token!(TokenType::Comma,      ",", self.line),
                '.' => return token!(TokenType::Dot,        ".", self.line),
                '-' => return token!(TokenType::Minus,      "-", self.line),
                '+' => return token!(TokenType::Plus,       "+", self.line),
                ';' => return token!(TokenType::Semicolon,  ";", self.line),
                '/' => return token!(TokenType::Slash,      "/", self.line),
                '*' => return token!(TokenType::Star,       "*", self.line),
                '!' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return token!(TokenType::BangEqual, "!=", self.line);
                } else {
                    return token!(TokenType::Bang, "!", self.line);
                },
                '=' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return token!(TokenType::EqualEqual, "==", self.line);
                } else {
                    return token!(TokenType::Equal, "=", self.line);
                },
                '>' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return token!(TokenType::GreaterEqual, ">=", self.line);
                } else {
                    return token!(TokenType::Greater, ">", self.line);
                },
                '<' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return token!(TokenType::LessEqual, "<=", self.line);
                } else {
                    return token!(TokenType::Less, "<", self.line);
                },
                _ => return token!(TokenType::Error, "", self.line),
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scanner_can_recognize_single_token() {
        let mut scanner = Scanner::new("=");
        assert_eq!(scanner.next(), Some(Token::new(TokenType::Equal, "=", 1)));
    }
}
