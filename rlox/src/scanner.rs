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
    pub fn from_type(kind: TokenType) -> Self {
        match kind {
            TokenType::LeftParen    => Token::new(TokenType::LeftParen, "("),
            TokenType::RightParen   => Token::new(TokenType::RightParen, ")"),
            TokenType::LeftBrace    => Token::new(TokenType::LeftBrace, "["),
            TokenType::RightBrace   => Token::new(TokenType::RightBrace, "]"),

            TokenType::Comma        => Token::new(TokenType::Comma, ","),
            TokenType::Dot          => Token::new(TokenType::Dot, "."),
            TokenType::Minus        => Token::new(TokenType::Minus, "-"),
            TokenType::Plus         => Token::new(TokenType::Plus, "+"),
            TokenType::Semicolon    => Token::new(TokenType::Semicolon, ";"),
            TokenType::Slash        => Token::new(TokenType::Slash, "/"),
            TokenType::Star         => Token::new(TokenType::Star, "*"),

            TokenType::Bang         => Token::new(TokenType::Bang, "!"),
            TokenType::BangEqual    => Token::new(TokenType::BangEqual, "!="),
            TokenType::Equal        => Token::new(TokenType::Equal, "="),
            TokenType::EqualEqual   => Token::new(TokenType::EqualEqual, "=="),
            TokenType::Greater      => Token::new(TokenType::Greater, ">"),
            TokenType::GreaterEqual => Token::new(TokenType::GreaterEqual, ">="),
            TokenType::Less         => Token::new(TokenType::Less, "<"),
            TokenType::LessEqual    => Token::new(TokenType::LessEqual, "<="),

            TokenType::Identifier   => Token::new(TokenType::Identifier, "i"),
            TokenType::Str          => Token::new(TokenType::Str, ""),
            TokenType::Number       => Token::new(TokenType::Number, "0"),

            TokenType::And          => Token::new(TokenType::And, "and"),
            TokenType::Class        => Token::new(TokenType::Class, "class"),
            TokenType::Else         => Token::new(TokenType::Else, "else"),
            TokenType::False        => Token::new(TokenType::False, "false"),
            TokenType::Fun          => Token::new(TokenType::Fun, "fun"),
            TokenType::For          => Token::new(TokenType::For, "for"),
            TokenType::If           => Token::new(TokenType::If, "if"),
            TokenType::Nil          => Token::new(TokenType::Nil, "nil"),
            TokenType::Or           => Token::new(TokenType::Or, "or"),
            TokenType::Print        => Token::new(TokenType::Print, "print"),
            TokenType::Return       => Token::new(TokenType::Return, "return"),
            TokenType::Super        => Token::new(TokenType::Super, "super"),
            TokenType::This         => Token::new(TokenType::This, "this"),
            TokenType::True         => Token::new(TokenType::True, "true"),
            TokenType::Var          => Token::new(TokenType::Var, "var"),
            TokenType::While        => Token::new(TokenType::While, "while"),

            TokenType::Error        => Token::new(TokenType::Error, "error"),
        }
    }

    fn new(kind: TokenType, contents: &str) -> Self {
        Token::with_line(kind, contents, 1)
    }

    fn with_line(kind: TokenType, contents: &str, line: u32) -> Self {
        Token { kind: kind, contents: String::from(contents), line: line }
    }
}

pub struct Scanner<'a> {
    contents: iter::Peekable<str::Chars<'a>>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { contents: source.chars().peekable(), line: 1 }
    }

    fn make_token(&self, kind: TokenType, contents: &str) -> Token {
        Token::with_line(kind, contents, self.line)
    }

    fn default_token(&self, kind: TokenType) -> Token {
        let tok = Token::from_type(kind);
        Token { line: self.line, ..tok }
    }

    fn word_starting_with(&mut self, ch: char) -> Token {
        let mut buf = String::new();
        buf.push(ch);
        buf += &self.rest_of_word();
        match ch {
            'a' if &buf == "and" => self.default_token(TokenType::And),
            'c' if &buf == "class" => self.default_token(TokenType::Class),
            'e' if &buf == "else" => self.default_token(TokenType::Else),
            'f' if &buf == "false" => self.default_token(TokenType::False),
            'f' if &buf == "for" => self.default_token(TokenType::For),
            'f' if &buf == "fun" => self.default_token(TokenType::Fun),
            'i' if &buf == "if" => self.default_token(TokenType::If),
            'n' if &buf == "nil"=> self.default_token(TokenType::Nil),
            'o' if &buf == "or" => self.default_token(TokenType::Or),
            'p' if &buf == "print" => self.default_token(TokenType::Print),
            'r' if &buf == "return" => self.default_token(TokenType::Return),
            's' if &buf == "super" => self.default_token(TokenType::Super),
            't' if &buf == "this" => self.default_token(TokenType::This),
            't' if &buf == "true" => self.default_token(TokenType::True),
            'v' if &buf == "var" => self.default_token(TokenType::Var),
            'w' if &buf == "while" => self.default_token(TokenType::While),
            _ => Token::with_line(TokenType::Identifier, &buf, self.line),
        }
    }

    fn rest_of_word(&mut self) -> String {
        let mut buf = String::new();
        let mut ch_ = self.contents.peek().map(|&c| c);
        while let Some(ch) = ch_ {
            match ch {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {
                    buf.push(ch);
                    let _ = self.contents.next(); // the value we just pushed
                },
                _ => break,
            }
            ch_ = self.contents.peek().map(|&c| c); // set up for next iteration
        }
        buf
    }
}

impl<'a> iter::Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.contents.next() {
            match ch {
                '(' => return Some(self.default_token(TokenType::LeftParen)),
                ')' => return Some(self.default_token(TokenType::RightParen)),
                '{' => return Some(self.default_token(TokenType::LeftBrace)),
                '}' => return Some(self.default_token(TokenType::RightBrace)),
                ',' => return Some(self.default_token(TokenType::Comma)),
                '.' => return Some(self.default_token(TokenType::Dot)),
                '-' => return Some(self.default_token(TokenType::Minus)),
                '+' => return Some(self.default_token(TokenType::Plus)),
                ';' => return Some(self.default_token(TokenType::Semicolon)),
                '/' => return Some(self.default_token(TokenType::Slash)),
                '*' => return Some(self.default_token(TokenType::Star)),
                '!' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return Some(self.default_token(TokenType::BangEqual));
                } else {
                    return Some(self.default_token(TokenType::Bang));
                },
                '=' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return Some(self.default_token(TokenType::EqualEqual));
                } else {
                    return Some(self.default_token(TokenType::Equal));
                },
                '>' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return Some(self.default_token(TokenType::GreaterEqual));
                } else {
                    return Some(self.default_token(TokenType::Greater));
                },
                '<' => if let Some('=') = self.contents.peek() {
                    let _ = self.contents.next();
                    return Some(self.default_token(TokenType::LessEqual));
                } else {
                    return Some(self.default_token(TokenType::Less));
                },
                '"' => {
                    let mut buf = String::new();
                    while let Some(c) = self.contents.next() {
                        if c == '"' { break; }
                        buf.push(c);
                    }
                    return Some(self.make_token(TokenType::Str, &buf));
                },
                'a'...'z' | 'A'...'Z' | '_' => return Some(self.word_starting_with(ch)),
                ' ' | '\t' => {}, // skip quietly
                '\n' => { self.line += 1; },
                _ => return Some(self.make_token(TokenType::Error, "Unrecognized character encountered")),
            }
        }
        None
    }
}

impl<'a> iter::FusedIterator for Scanner<'a> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scanner_can_recognize_single_token() {
        let mut scanner = Scanner::new("=");
        assert_eq!(scanner.next(), Some(Token::from_type(TokenType::Equal)));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_can_recognize_multiple_tokens() {
        let mut scanner = Scanner::new("><");
        assert_eq!(scanner.next(), Some(Token::from_type(TokenType::Greater)));
        assert_eq!(scanner.next(), Some(Token::from_type(TokenType::Less)));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_can_skip_whitespace() {
        let mut scanner = Scanner::new("! !");
        assert_eq!(scanner.next(), Some(Token::from_type(TokenType::Bang)));
        assert_eq!(scanner.next(), Some(Token::from_type(TokenType::Bang)));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_can_recognize_string_literal() {
        let mut scanner = Scanner::new("\"hello\"");
        assert_eq!(scanner.next(), Some(Token::new(TokenType::Str, "hello")));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_can_recognize_identifier() {
        let mut scanner = Scanner::new("id");
        assert_eq!(scanner.next(), Some(Token::new(TokenType::Identifier, "id")));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_can_recognize_true() {
        let mut scanner = Scanner::new("true");
        assert_eq!(scanner.next(), Some(Token::from_type(TokenType::True)));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_can_recognize_super() {
        let mut scanner = Scanner::new("super");
        assert_eq!(scanner.next(), Some(Token::from_type(TokenType::Super)));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn scanner_does_not_recognize_super_inside_superb() {
        let mut scanner = Scanner::new("superb");
        assert_eq!(scanner.next(), Some(Token::new(TokenType::Identifier, "superb")));
        assert_eq!(scanner.next(), None);
    }
}
